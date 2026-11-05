use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use file_fixer_dev::file_fixer::corrupt::{ByteSpan, DamageKind, DamageSpec, Fraction, standard_corpus};
use file_fixer_dev::file_fixer::engine::{Budget, RepairOptions};
use file_fixer_dev::file_fixer::mode::{CapabilityOverrides, Mode};

use crate::progress::Visibility;

#[derive(Parser, Debug)]
#[command(name = "file_fixer", version, about = "Repairs damaged media files", long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Repairing is the default action, so `file_fixer photo.jpg` needs no subcommand.
    #[command(flatten)]
    pub repair: RepairArgs,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Repair damaged files (the default when no subcommand is given).
    Repair(RepairArgs),
    /// Build a damaged corpus from healthy files, for developing and benchmarking repairs.
    Corrupt(CorruptArgs),
    /// Find damaged files without repairing them, copying them into `<out>/broken`.
    FindBroken(FindArgs),
    /// Find damaged files whose format is recognized but a trial repair could not fix,
    /// copying them into `<out>/unsupported`. Nothing is written to the input or a repaired
    /// copy; the repair is only attempted in memory to see whether it would succeed.
    FindUnsupported(FindArgs),
    /// Read encoder configurations out of healthy recordings and remember them, so that later
    /// repairs of files from the same camera need no `--reference`.
    ScanProfiles(ScanProfilesArgs),
}

#[derive(Args, Debug, Default)]
pub struct ScanProfilesArgs {
    /// Files or directories to read configurations from. Damaged files are fine: only the
    /// encoder configuration is taken, and that survives most damage. Not needed with `--list`.
    pub paths: Vec<PathBuf>,

    /// Store the configurations here instead of in the cache directory.
    #[arg(long, value_name = "FILE")]
    pub profile_store: Option<PathBuf>,

    /// List what is remembered instead of adding to it.
    #[arg(long)]
    pub list: bool,

    #[arg(short, long, value_name = "N", default_value_t = 0)]
    pub jobs: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Capability {
    /// Discard data that cannot be validated.
    Truncate,
    /// Re-encode content, losing generational quality.
    Reencode,
    /// Reconstruct structures that are missing entirely.
    Guess,
    /// Emit synthetic content in place of unrecoverable data.
    FillGaps,
}

#[derive(Args, Debug, Default)]
pub struct RepairArgs {
    /// Files or directories to repair.
    #[arg(value_name = "PATH")]
    pub paths: Vec<PathBuf>,

    /// Descend into directories.
    #[arg(short, long)]
    pub recursive: bool,

    /// How much the repair is allowed to do. Use `balanced` to keep every result byte-faithful.
    #[arg(short, long, default_value = "aggressive")]
    pub mode: ModeArg,

    /// Grant a capability the mode does not include.
    #[arg(long = "allow", value_name = "CAPABILITY")]
    pub allow: Vec<Capability>,

    /// Revoke a capability the mode includes.
    #[arg(long = "deny", value_name = "CAPABILITY")]
    pub deny: Vec<Capability>,

    /// Files of the same provenance, used to supply structures the damaged file lost. Video
    /// only. The reference may itself be damaged; only its encoder configuration is read.
    #[arg(long, value_name = "FILE")]
    pub reference: Vec<PathBuf>,

    /// Skip the pass that learns encoder configurations from the batch before repairing it.
    #[arg(long)]
    pub no_learn: bool,

    /// Also learn encoder configurations from these files or directories. Use when repairing a
    /// few files out of a collection whose other members still carry their indexes.
    #[arg(long, value_name = "PATH")]
    pub learn_from: Vec<PathBuf>,

    /// Remember encoder configurations here instead of in the cache directory. See
    /// `scan-profiles`.
    #[arg(long, value_name = "FILE")]
    pub profile_store: Option<PathBuf>,

    /// Write results here instead of next to the input.
    #[arg(short, long, value_name = "DIR")]
    pub output_dir: Option<PathBuf>,

    /// Keep every candidate, not just the winner.
    #[arg(long, value_name = "DIR")]
    pub keep_candidates: Option<PathBuf>,

    /// Overwrite an existing output file.
    #[arg(long)]
    pub force: bool,

    /// Seconds to spend per file. 0 removes the limit.
    #[arg(long, default_value_t = 60)]
    pub budget: u64,

    /// Maximum number of candidates per file. 0 removes the limit.
    #[arg(long, default_value_t = 64)]
    pub max_candidates: usize,

    /// Generate every candidate instead of stopping at the first perfect one.
    #[arg(long)]
    pub exhaustive: bool,

    /// Fraction of the leading candidate's content a byte-faithful candidate must reach to win.
    #[arg(long, default_value_t = 0.9)]
    pub lossless_threshold: f64,

    /// Worker threads. 0 uses one per core.
    #[arg(short, long, default_value_t = 0)]
    pub jobs: usize,

    /// Emit a machine-readable report instead of one line per file.
    #[arg(long)]
    pub json: bool,

    /// Show every candidate and why the winner was chosen.
    #[arg(short, long)]
    pub verbose: bool,

    /// Live progress bar on stderr. `auto` turns it on for an interactive terminal.
    #[arg(long, default_value = "auto", value_name = "WHEN")]
    pub progress: ProgressArg,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
pub enum ProgressArg {
    #[default]
    Auto,
    Always,
    Never,
}

impl From<ProgressArg> for Visibility {
    fn from(value: ProgressArg) -> Self {
        match value {
            ProgressArg::Auto => Self::Auto,
            ProgressArg::Always => Self::Always,
            ProgressArg::Never => Self::Never,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
pub enum ModeArg {
    Safe,
    Balanced,
    /// The default: a recovery tool that leaves content on the table because a flag was not
    /// typed is not doing its job. The fidelity label on every candidate, and the lossless
    /// preference in the selection rule, are what keep the extra freedom honest.
    #[default]
    Aggressive,
}

impl From<ModeArg> for Mode {
    fn from(value: ModeArg) -> Self {
        match value {
            ModeArg::Safe => Self::Safe,
            ModeArg::Balanced => Self::Balanced,
            ModeArg::Aggressive => Self::Aggressive,
        }
    }
}

impl RepairArgs {
    pub fn to_options(&self) -> RepairOptions {
        RepairOptions {
            mode: self.mode.into(),
            overrides: self.capability_overrides(),
            budget: Budget {
                deadline: (self.budget > 0).then(|| std::time::Duration::from_secs(self.budget)),
                max_candidates: (self.max_candidates > 0).then_some(self.max_candidates),
                exhaustive: self.exhaustive,
            },
            lossless_threshold: self.lossless_threshold.clamp(0.0, 1.0),
            allocation_limit: file_fixer_dev::file_fixer::io::DEFAULT_ALLOCATION_LIMIT,
            references: self.reference.clone(),
            output_dir: self.output_dir.clone(),
            keep_candidates: self.keep_candidates.clone(),
            force: self.force,
        }
    }

    fn capability_overrides(&self) -> CapabilityOverrides {
        capability_overrides(&self.allow, &self.deny)
    }
}

/// `--deny` is applied after `--allow`, so denying always wins a conflict.
fn capability_overrides(allow: &[Capability], deny: &[Capability]) -> CapabilityOverrides {
    let mut overrides = CapabilityOverrides::default();
    for capability in allow {
        set_capability(&mut overrides, *capability, true);
    }
    for capability in deny {
        set_capability(&mut overrides, *capability, false);
    }
    overrides
}

fn set_capability(overrides: &mut CapabilityOverrides, capability: Capability, value: bool) {
    match capability {
        Capability::Truncate => overrides.truncate = Some(value),
        Capability::Reencode => overrides.reencode = Some(value),
        Capability::Guess => overrides.guess = Some(value),
        Capability::FillGaps => overrides.fill_gaps = Some(value),
    }
}

#[derive(Args, Debug)]
pub struct FindArgs {
    /// Files or directories to scan.
    #[arg(value_name = "PATH", required = true)]
    pub paths: Vec<PathBuf>,

    /// Descend into directories.
    #[arg(short, long)]
    pub recursive: bool,

    /// How hard the trial repair tries before giving up on a file.
    #[arg(short, long, default_value = "aggressive")]
    pub mode: ModeArg,

    /// Grant a capability the mode does not include.
    #[arg(long = "allow", value_name = "CAPABILITY")]
    pub allow: Vec<Capability>,

    /// Revoke a capability the mode includes.
    #[arg(long = "deny", value_name = "CAPABILITY")]
    pub deny: Vec<Capability>,

    /// Files of the same provenance, used to supply structures the damaged file lost. Video
    /// only. The reference may itself be damaged; only its encoder configuration is read.
    #[arg(long, value_name = "FILE")]
    pub reference: Vec<PathBuf>,

    /// Skip the pass that learns encoder configurations from the batch before checking it.
    #[arg(long)]
    pub no_learn: bool,

    /// Also learn encoder configurations from these files or directories.
    #[arg(long, value_name = "PATH")]
    pub learn_from: Vec<PathBuf>,

    /// Remember encoder configurations here instead of in the cache directory.
    #[arg(long, value_name = "FILE")]
    pub profile_store: Option<PathBuf>,

    /// Directory matching files are copied into, under a subfolder named after the check
    /// (`broken` or `unsupported`).
    #[arg(short, long, value_name = "DIR")]
    pub out: PathBuf,

    /// Seconds to spend per file on the trial repair. 0 removes the limit.
    #[arg(long, default_value_t = 60)]
    pub budget: u64,

    /// Maximum number of candidates per file. 0 removes the limit.
    #[arg(long, default_value_t = 64)]
    pub max_candidates: usize,

    /// Worker threads. 0 uses one per core.
    #[arg(short, long, default_value_t = 0)]
    pub jobs: usize,

    /// Emit a machine-readable report instead of one line per file.
    #[arg(long)]
    pub json: bool,

    /// Live progress bar on stderr. `auto` turns it on for an interactive terminal.
    #[arg(long, default_value = "auto", value_name = "WHEN")]
    pub progress: ProgressArg,
}

impl FindArgs {
    /// The trial repair never writes anywhere: `output_dir`, `keep_candidates` and `force`
    /// stay at their defaults regardless of what the caller passes.
    pub fn to_options(&self) -> RepairOptions {
        RepairOptions {
            mode: self.mode.into(),
            overrides: capability_overrides(&self.allow, &self.deny),
            budget: Budget {
                deadline: (self.budget > 0).then(|| std::time::Duration::from_secs(self.budget)),
                max_candidates: (self.max_candidates > 0).then_some(self.max_candidates),
                exhaustive: false,
            },
            references: self.reference.clone(),
            ..RepairOptions::default()
        }
    }
}

#[derive(Args, Debug)]
pub struct CorruptArgs {
    /// Healthy files to damage.
    #[arg(value_name = "PATH", required = true)]
    pub paths: Vec<PathBuf>,

    /// Where the damaged copies go.
    #[arg(short, long, value_name = "DIR")]
    pub output_dir: PathBuf,

    /// Descend into directories.
    #[arg(short, long)]
    pub recursive: bool,

    /// Seed for the damage generator. The same seed always produces the same bytes.
    #[arg(long, default_value_t = 0)]
    pub seed: u64,

    /// Keep only this fraction of the file, instead of the standard damage set.
    #[arg(long, value_name = "FRACTION")]
    pub truncate: Option<f64>,

    /// Zero this many bytes in the middle, instead of the standard damage set.
    #[arg(long, value_name = "BYTES")]
    pub zero_bytes: Option<u64>,
}

impl CorruptArgs {
    /// Explicit flags replace the standard damage set; without them every class is generated.
    pub fn damage_specs(&self) -> Vec<DamageSpec> {
        let mut specs = Vec::new();

        if let Some(keep) = self.truncate.and_then(Fraction::new) {
            specs.push(DamageSpec::new(DamageKind::TruncateTail { keep }, self.seed));
        }
        if let Some(len) = self.zero_bytes {
            specs.push(DamageSpec::new(
                DamageKind::ZeroRange {
                    at: Fraction::HALF,
                    len: ByteSpan::Bytes(len),
                },
                self.seed,
            ));
        }

        if specs.is_empty() {
            standard_corpus(self.seed)
        } else {
            specs
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;
    use file_fixer_dev::file_fixer::mode::Capabilities;

    use super::*;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn a_bare_path_repairs_without_a_subcommand() {
        let cli = Cli::try_parse_from(["file_fixer", "photo.png"]).unwrap();
        assert!(cli.command.is_none());
        assert_eq!(cli.repair.paths, vec![PathBuf::from("photo.png")]);
    }

    /// A deliberate choice rather than an accident of ordering: a bare invocation recovers as
    /// much as the data allows, and a mode that only produces byte-faithful output is opt-in.
    #[test]
    fn a_bare_invocation_is_allowed_to_do_everything() {
        let cli = Cli::try_parse_from(["file_fixer", "clip.mp4"]).unwrap();
        let options = cli.repair.to_options();

        assert_eq!(options.mode, Mode::Aggressive);
        assert_eq!(options.capabilities(), file_fixer_dev::file_fixer::mode::Capabilities::ALL);
        assert_eq!(
            Cli::try_parse_from(["file_fixer", "--mode", "balanced", "clip.mp4"])
                .unwrap()
                .repair
                .to_options()
                .mode,
            Mode::Balanced
        );
    }

    #[test]
    fn allow_grants_a_capability_the_mode_lacks() {
        let cli = Cli::try_parse_from(["file_fixer", "--mode", "safe", "--allow", "truncate", "x.png"]).unwrap();
        let caps = cli.repair.to_options().capabilities();

        assert!(caps.truncate);
        assert!(!caps.reencode);
    }

    #[test]
    fn deny_beats_allow_for_the_same_capability() {
        let cli = Cli::try_parse_from([
            "file_fixer",
            "--mode",
            "aggressive",
            "--allow",
            "guess",
            "--deny",
            "guess",
            "x.png",
        ])
        .unwrap();
        assert!(!cli.repair.to_options().capabilities().guess);
    }

    #[test]
    fn aggressive_mode_grants_everything_by_default() {
        let cli = Cli::try_parse_from(["file_fixer", "--mode", "aggressive", "x.png"]).unwrap();
        assert_eq!(cli.repair.to_options().capabilities(), Capabilities::ALL);
    }

    #[test]
    fn a_zero_budget_means_no_deadline() {
        let cli = Cli::try_parse_from(["file_fixer", "--budget", "0", "--max-candidates", "0", "x.png"]).unwrap();
        let budget = cli.repair.to_options().budget;

        assert!(budget.deadline.is_none());
        assert!(budget.max_candidates.is_none());
    }

    #[test]
    fn an_out_of_range_threshold_is_clamped() {
        let cli = Cli::try_parse_from(["file_fixer", "--lossless-threshold", "5.0", "x.png"]).unwrap();
        assert!((cli.repair.to_options().lossless_threshold - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn corrupt_defaults_to_the_standard_damage_set() {
        let cli = Cli::try_parse_from(["file_fixer", "corrupt", "-o", "/tmp/out", "a.png"]).unwrap();
        let Some(Command::Corrupt(args)) = cli.command else {
            panic!("expected the corrupt subcommand");
        };
        assert_eq!(args.damage_specs().len(), standard_corpus(0).len());
    }

    #[test]
    fn find_broken_requires_an_out_directory() {
        Cli::try_parse_from(["file_fixer", "find-broken", "a.png"]).unwrap_err();
        let cli = Cli::try_parse_from(["file_fixer", "find-broken", "--out", "out/", "a.png"]).unwrap();
        let Some(Command::FindBroken(args)) = cli.command else {
            panic!("expected the find-broken subcommand");
        };
        assert_eq!(args.out, PathBuf::from("out/"));
    }

    #[test]
    fn find_unsupported_parses_like_find_broken() {
        let cli = Cli::try_parse_from(["file_fixer", "find-unsupported", "-o", "out/", "a.mp4"]).unwrap();
        assert!(matches!(cli.command, Some(Command::FindUnsupported(_))));
    }

    #[test]
    fn an_explicit_damage_flag_replaces_the_standard_set() {
        let cli =
            Cli::try_parse_from(["file_fixer", "corrupt", "-o", "/tmp/out", "--truncate", "0.5", "a.png"]).unwrap();
        let Some(Command::Corrupt(args)) = cli.command else {
            panic!("expected the corrupt subcommand");
        };
        let specs = args.damage_specs();

        assert_eq!(specs.len(), 1);
        assert!(matches!(specs[0].kind, DamageKind::TruncateTail { .. }));
    }
}
