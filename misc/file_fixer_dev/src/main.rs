//! Command line front end. All user-facing printing lives here; the library only returns data.

#![expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "this is the CLI layer, writing to the terminal is its job"
)]

mod cli;
mod progress;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use cli::{Cli, Command, CorruptArgs, FindArgs, RepairArgs, ScanProfilesArgs};
use file_fixer_dev::file_fixer::corrupt::apply_damage;
use file_fixer_dev::file_fixer::engine::{RepairOptions, repair_data, repair_file};
use file_fixer_dev::file_fixer::format::Format;
use file_fixer_dev::file_fixer::io::SourceFile;
use file_fixer_dev::file_fixer::output::{fixed_output_path, is_fixed_output_name};
use file_fixer_dev::file_fixer::report::{Outcome, RepairReport};
use jwalk::WalkDir;
use progress::Progress;
use rayon::prelude::*;

fn main() -> ExitCode {
    if let Err(error) = handsome_logger::init() {
        eprintln!("warning: could not install the logger: {error}");
    }

    let cli = Cli::parse();
    match cli.command {
        Some(Command::Corrupt(args)) => run_corrupt(&args),
        Some(Command::Repair(args)) => run_repair(&args),
        Some(Command::FindBroken(args)) => run_find(&args, FindKind::Broken),
        Some(Command::FindUnsupported(args)) => run_find(&args, FindKind::Unsupported),
        Some(Command::ScanProfiles(args)) => run_scan_profiles(&args),
        None => run_repair(&cli.repair),
    }
}

fn run_repair(args: &RepairArgs) -> ExitCode {
    if args.paths.is_empty() {
        eprintln!("no input paths given; run with --help for usage");
        return ExitCode::from(2);
    }
    configure_thread_pool(args.jobs);

    eprintln!("scanning {} path(s)...", args.paths.len());
    let files = collect_files(&args.paths, args.recursive);
    let (files, skipped) = skip_already_fixed(files, args.output_dir.as_deref());
    if skipped > 0 {
        eprintln!("skipping {skipped} file(s) already fixed (name carries the marker, or an output already exists)");
    }
    if files.is_empty() {
        eprintln!("no files found in the given paths");
        return ExitCode::from(2);
    }

    warn_if_video_support_is_missing(&files);

    let options = args.to_options();
    let sizes = file_sizes(&files);
    print_inventory(&files, &sizes, &options, args);
    if !args.no_learn {
        let mut corpus = files.clone();
        corpus.extend(collect_files(&args.learn_from, true));
        corpus.sort_unstable();
        corpus.dedup();
        prepare_profiles(
            &corpus,
            profile_store(args.no_learn, args.profile_store.as_deref()).as_deref(),
        );
    }

    // JSON has to stay a single well-formed document, so its reports are still collected and
    // sorted; the text path prints each file the moment it is done.
    let progress = Progress::new(
        files.len(),
        sizes.iter().sum(),
        if args.json {
            progress::Visibility::Never
        } else {
            args.progress.into()
        },
    );

    let mut reports: Vec<RepairReport> = files
        .par_iter()
        .zip(sizes.par_iter())
        .map(|(path, &size)| {
            progress.file_started(path);
            let report = repair_file(path, &options);
            if args.json {
                progress.file_finished(size, "");
            } else {
                progress.file_finished(size, &report.summary_line());
                if args.verbose {
                    print_candidates(&report, &progress);
                }
            }
            report
        })
        .collect();
    progress.finish();
    reports.sort_by(|a, b| a.path.cmp(&b.path));

    if args.json {
        print_json(&reports);
    } else {
        print_totals(&reports, &options, progress.elapsed());
    }
    ExitCode::from(worst_outcome(&reports).exit_code() as u8)
}

/// Drops inputs that are themselves a previous output, or that already have one sitting next
/// to them, so a repeated run over the same tree does not redo work it already did.
fn skip_already_fixed(files: Vec<PathBuf>, output_dir: Option<&Path>) -> (Vec<PathBuf>, usize) {
    let mut skipped = 0;
    let kept = files
        .into_iter()
        .filter(|path| {
            let already_done = is_fixed_output_name(path) || fixed_output_path(path, output_dir).exists();
            if already_done {
                skipped += 1;
            }
            !already_done
        })
        .collect();
    (kept, skipped)
}

#[derive(Clone, Copy)]
enum FindKind {
    /// Any file that did not come back healthy from a trial repair.
    Broken,
    /// A broken file whose format is recognized but the trial repair produced no candidate.
    Unsupported,
}

impl FindKind {
    fn subdir(self) -> &'static str {
        match self {
            Self::Broken => "broken",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Scans for damaged files and copies the matching originals into `<out>/<kind>`. Nothing is
/// repaired or written back over the input; the repair attempt only runs in memory to decide
/// whether the file qualifies.
fn run_find(args: &FindArgs, kind: FindKind) -> ExitCode {
    if args.paths.is_empty() {
        eprintln!("no input paths given; run with --help for usage");
        return ExitCode::from(2);
    }
    configure_thread_pool(args.jobs);

    eprintln!("scanning {} path(s)...", args.paths.len());
    let files = collect_files(&args.paths, args.recursive);
    let (files, skipped) = skip_already_fixed(files, None);
    if skipped > 0 {
        eprintln!("skipping {skipped} file(s) already fixed (name carries the marker, or an output already exists)");
    }
    if files.is_empty() {
        eprintln!("no files found in the given paths");
        return ExitCode::from(2);
    }

    warn_if_video_support_is_missing(&files);

    let options = args.to_options();
    let sizes = file_sizes(&files);
    eprintln!("found {} file(s), {}", files.len(), progress::bytes(sizes.iter().sum()));
    if !args.no_learn {
        let mut corpus = files.clone();
        corpus.extend(collect_files(&args.learn_from, true));
        corpus.sort_unstable();
        corpus.dedup();
        prepare_profiles(
            &corpus,
            profile_store(args.no_learn, args.profile_store.as_deref()).as_deref(),
        );
    }

    let destination = args.out.join(kind.subdir());
    if let Err(error) = std::fs::create_dir_all(&destination) {
        eprintln!("cannot create {}: {error}", destination.display());
        return ExitCode::from(2);
    }

    let progress = Progress::new(
        files.len(),
        sizes.iter().sum(),
        if args.json {
            progress::Visibility::Never
        } else {
            args.progress.into()
        },
    );

    let matches: Vec<&PathBuf> = files
        .par_iter()
        .zip(sizes.par_iter())
        .filter_map(|(path, &size)| {
            progress.file_started(path);
            let matched = matches_find_kind(path, &options, kind);
            if !args.json {
                let verdict = if matched { kind.subdir() } else { "ok" };
                progress.file_finished(size, &format!("{}: {verdict}", path.display()));
            } else {
                progress.file_finished(size, "");
            }
            matched.then_some(path)
        })
        .collect();
    progress.finish();

    let mut copied = 0usize;
    for &path in &matches {
        match copy_into(path, &destination) {
            Ok(()) => copied += 1,
            Err(error) => eprintln!("cannot copy {}: {error}", path.display()),
        }
    }

    if args.json {
        match serde_json::to_string_pretty(&matches) {
            Ok(json) => println!("{json}"),
            Err(error) => eprintln!("failed to serialize the match list: {error}"),
        }
    } else {
        println!(
            "\n{} of {} file(s) in {} are {}, {copied} copied into {}",
            matches.len(),
            files.len(),
            progress::duration(progress.elapsed()),
            kind.subdir(),
            destination.display()
        );
    }
    ExitCode::SUCCESS
}

/// Runs the trial repair entirely in memory - the input is read but never written to, and no
/// repaired copy is produced on disk - then classifies the result for `kind`.
fn matches_find_kind(path: &Path, options: &RepairOptions, kind: FindKind) -> bool {
    // Unreadable counts as broken, but not as "recognized format the repair failed on".
    let Ok(source) = SourceFile::open(path) else {
        return matches!(kind, FindKind::Broken);
    };
    let result = repair_data(path, source.data(), options);
    let broken = result.report.outcome != Outcome::Healthy;
    match kind {
        FindKind::Broken => broken,
        FindKind::Unsupported => broken && result.report.format.is_some() && result.winner.is_none(),
    }
}

/// Copies `path` into `directory`, keeping the original name unless it collides with a file
/// already copied there, in which case a numeric suffix is appended.
fn copy_into(path: &Path, directory: &Path) -> std::io::Result<()> {
    let stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let extension = path.extension().map(|e| e.to_string_lossy().to_string());
    let mut target = directory.join(path.file_name().unwrap_or_default());

    let mut counter = 1;
    while target.exists() {
        counter += 1;
        let mut name = format!("{stem}__{counter}");
        if let Some(extension) = &extension {
            name.push('.');
            name.push_str(extension);
        }
        target = directory.join(name);
    }

    std::fs::copy(path, &target)?;
    Ok(())
}

/// Builds a reference library out of the batch before repairing any of it.
///
/// A video whose index is gone carries no description of its own encoding, and roughly half of
/// a carved collection is in that state. Its neighbours are not: the clips came off the same
/// few devices, so the configuration the broken file needs is usually sitting in a sibling that
/// kept its index. One pass over the inputs collects them all.
///
/// The pass is mapped rather than read, so it costs page cache rather than memory, and it is
/// skipped entirely for a single file - there would be nothing to learn from.
fn learn_encoder_profiles(files: &[PathBuf]) {
    let videos: Vec<&PathBuf> = files
        .iter()
        .filter(|path| file_fixer_dev::file_fixer::format::detect_by_extension(path) == Some(Format::Mp4))
        .collect();
    if videos.len() < 2 {
        return;
    }

    eprint!("learning encoder profiles from {} video(s)...", videos.len());
    harvest_profiles(&videos);
    let learned = file_fixer_dev::file_fixer::formats::mp4_profile::learned().len();
    eprintln!(" {learned} distinct");
}

fn harvest_profiles(videos: &[&PathBuf]) {
    let harvested: Vec<_> = videos
        .par_iter()
        .flat_map(|path| file_fixer_dev::file_fixer::formats::mp4_profile::harvest_file(path))
        .collect();
    file_fixer_dev::file_fixer::formats::mp4_profile::learn(harvested);
}

/// Everything the profile library needs before a batch runs: what earlier runs remembered,
/// then what this batch can teach it, then back to disk.
///
/// Reading the store is what makes repairing ten files out of a collection of ten thousand
/// work at all - the configuration a broken file needs came from a camera, and a camera does
/// not change between Tuesday and Thursday.
fn prepare_profiles(corpus: &[PathBuf], store: Option<&Path>) {
    if let Some(path) = store {
        let remembered = file_fixer_dev::file_fixer::formats::mp4_profile_store::load(path);
        if remembered > 0 {
            eprintln!(
                "recalling {remembered} encoder configuration(s) from {}",
                path.display()
            );
        }
    }
    learn_encoder_profiles(corpus);
    if let Some(path) = store
        && let Err(error) = file_fixer_dev::file_fixer::formats::mp4_profile_store::save(path)
    {
        eprintln!("warning: could not update {}: {error}", path.display());
    }
}

/// Reads encoder configurations out of healthy recordings and remembers them.
///
/// The point is to do the expensive part once. A configuration describes the camera, not the
/// recording, so scanning a folder of good footage from a phone makes every damaged file that
/// phone ever produced repairable without a `--reference` - today and next year.
fn run_scan_profiles(args: &ScanProfilesArgs) -> ExitCode {
    let Some(store) = args
        .profile_store
        .clone()
        .or_else(file_fixer_dev::file_fixer::formats::mp4_profile_store::default_path)
    else {
        eprintln!("no cache directory found; pass --profile-store to say where to keep the configurations");
        return ExitCode::from(2);
    };

    if args.list {
        return list_profiles(&store);
    }
    if args.paths.is_empty() {
        eprintln!("no input paths given; run with --help for usage");
        return ExitCode::from(2);
    }
    configure_thread_pool(args.jobs);

    let files = collect_files(&args.paths, true);
    let videos: Vec<&PathBuf> = files
        .iter()
        .filter(|path| file_fixer_dev::file_fixer::format::detect_by_extension(path) == Some(Format::Mp4))
        .collect();
    if videos.is_empty() {
        eprintln!("no MP4-family recordings found in the given paths");
        return ExitCode::from(2);
    }

    let before = file_fixer_dev::file_fixer::formats::mp4_profile_store::read(&store).profiles.len();
    file_fixer_dev::file_fixer::formats::mp4_profile_store::load(&store);
    eprintln!("reading {} recording(s)...", videos.len());
    harvest_profiles(&videos);

    match file_fixer_dev::file_fixer::formats::mp4_profile_store::save(&store) {
        Ok(total) => {
            println!(
                "{total} encoder configuration(s) remembered in {} ({} new)",
                store.display(),
                total.saturating_sub(before)
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("cannot write {}: {error}", store.display());
            ExitCode::from(2)
        }
    }
}

fn list_profiles(store: &Path) -> ExitCode {
    let held = file_fixer_dev::file_fixer::formats::mp4_profile_store::read(store);
    if held.is_empty() {
        println!("nothing remembered yet in {}", store.display());
        return ExitCode::SUCCESS;
    }

    println!("{}:", store.display());
    for (index, profile) in held.profiles.iter().enumerate() {
        println!(
            "  {index:>3}. {:?}, {} parameter set(s), samples up to {}, {:.2} fps",
            profile.codec,
            profile.parameter_sets().count(),
            progress::bytes(u64::from(profile.max_sample_size)),
            profile.frame_rate
        );
    }
    if !held.mpeg4_headers.is_empty() {
        println!("  and {} MPEG-4 Part 2 sequence header(s)", held.mpeg4_headers.len());
    }
    ExitCode::SUCCESS
}

/// The store a run should use, unless it was told not to keep one.
fn profile_store(no_learn: bool, named: Option<&Path>) -> Option<PathBuf> {
    if no_learn {
        return None;
    }
    named
        .map(Path::to_path_buf)
        .or_else(file_fixer_dev::file_fixer::formats::mp4_profile_store::default_path)
}

/// Sizes are read once up front so the progress estimate has a denominator, and because a
/// stat per file is nothing next to the repairs themselves.
fn file_sizes(files: &[PathBuf]) -> Vec<u64> {
    files
        .iter()
        .map(|path| std::fs::metadata(path).map_or(0, |meta| meta.len()))
        .collect()
}

/// What the run is about to do, printed before any work starts.
///
/// On a large directory this is the only output for a while, and it is what tells the user
/// that the tool found their files at all - the previous silence was indistinguishable from
/// a hang or an empty match.
fn print_inventory(files: &[PathBuf], sizes: &[u64], options: &RepairOptions, args: &RepairArgs) {
    let total: u64 = sizes.iter().sum();
    eprintln!("found {} file(s), {}", files.len(), progress::bytes(total));

    let mut counts: Vec<(String, usize)> = Vec::new();
    for path in files {
        let name = file_fixer_dev::file_fixer::format::detect_by_extension(path).map_or_else(|| "other".to_string(), format_name);
        match counts.iter_mut().find(|(existing, _)| *existing == name) {
            Some((_, count)) => *count += 1,
            None => counts.push((name, 1)),
        }
    }
    counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let breakdown: Vec<String> = counts.iter().map(|(name, count)| format!("{count} {name}")).collect();
    eprintln!("  {}", breakdown.join(", "));

    let budget = match options.budget.deadline {
        Some(deadline) => format!("{}s per file", deadline.as_secs()),
        None => "unlimited".to_string(),
    };
    let jobs = if args.jobs == 0 {
        rayon::current_num_threads()
    } else {
        args.jobs
    };
    eprintln!("  mode {}, {jobs} worker(s), budget {budget}", options.mode);
}

fn format_name(format: Format) -> String {
    format.to_string()
}

/// Video repair goes through ffmpeg, so its absence turns every clip into an unexplained
/// failure unless it is called out up front.
fn warn_if_video_support_is_missing(files: &[PathBuf]) {
    if file_fixer_dev::file_fixer::formats::ffmpeg::is_available() {
        return;
    }
    let videos = files
        .iter()
        .filter(|path| {
            matches!(
                file_fixer_dev::file_fixer::format::detect_by_extension(path),
                Some(file_fixer_dev::file_fixer::Format::Mp4 | file_fixer_dev::file_fixer::Format::Avi)
            )
        })
        .count();
    if videos > 0 {
        eprintln!("warning: ffmpeg was not found in PATH, so {videos} video file(s) cannot be repaired");
    }
}

fn print_json(reports: &[RepairReport]) {
    match serde_json::to_string_pretty(reports) {
        Ok(json) => println!("{json}"),
        Err(error) => eprintln!("failed to serialize the report: {error}"),
    }
}

fn print_candidates(report: &RepairReport, progress: &Progress) {
    if report.budget_exhausted {
        progress.print("    budget exhausted, a better candidate may exist");
    }
    for candidate in &report.candidates {
        let marker = if candidate.selected { "->" } else { "  " };
        let total = candidate
            .score
            .content_total
            .map_or_else(|| "?".to_string(), |t| t.to_string());
        progress.print(&format!(
            "    {marker} {:<28} {}/{total} content, {} error(s), {:?}",
            candidate.strategy, candidate.score.content_recovered, candidate.score.decoder_errors, candidate.fidelity
        ));
    }
    if report.lossless_preferred {
        progress.print("    chose a byte-faithful candidate over one that recovered more");
    }
}

fn print_totals(reports: &[RepairReport], options: &RepairOptions, elapsed: std::time::Duration) {
    let count = |outcome: Outcome| reports.iter().filter(|r| r.outcome == outcome).count();
    println!(
        "\n{} file(s) in {}, mode {}: {} healthy, {} repaired, {} partial, {} unrecoverable, {} unsupported",
        reports.len(),
        progress::duration(elapsed),
        options.mode,
        count(Outcome::Healthy),
        count(Outcome::FullyRepaired),
        count(Outcome::PartiallyRecovered),
        count(Outcome::Unrecoverable),
        count(Outcome::Unsupported),
    );

    // The files a user actually needs to look at, listed again so they are not lost among
    // hundreds of healthy ones that scrolled past.
    let failed: Vec<&RepairReport> = reports
        .iter()
        .filter(|report| matches!(report.outcome, Outcome::Unrecoverable | Outcome::PartiallyRecovered))
        .collect();
    if !failed.is_empty() && failed.len() < reports.len() {
        println!("\nnot fully recovered:");
        for report in failed {
            println!("  {}", report.summary_line());
        }
    }
}

fn run_corrupt(args: &CorruptArgs) -> ExitCode {
    if let Err(error) = std::fs::create_dir_all(&args.output_dir) {
        eprintln!("cannot create {}: {error}", args.output_dir.display());
        return ExitCode::from(2);
    }

    let files = collect_files(&args.paths, args.recursive);
    if files.is_empty() {
        eprintln!("no files found in the given paths");
        return ExitCode::from(2);
    }

    let specs = args.damage_specs();
    let written: usize = files
        .par_iter()
        .map(|path| write_damaged_copies(path, &specs, &args.output_dir))
        .sum();

    println!(
        "wrote {written} damaged file(s) from {} input(s) into {}",
        files.len(),
        args.output_dir.display()
    );
    ExitCode::SUCCESS
}

fn write_damaged_copies(path: &Path, specs: &[file_fixer_dev::file_fixer::corrupt::DamageSpec], output_dir: &Path) -> usize {
    let Ok(original) = std::fs::read(path) else {
        eprintln!("cannot read {}", path.display());
        return 0;
    };

    let stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let extension = path.extension().map(|e| e.to_string_lossy().to_string());

    specs
        .iter()
        .filter(|spec| {
            let damaged = apply_damage(&original, spec);
            let mut name = format!("{stem}__{}", spec.label());
            if let Some(extension) = &extension {
                name.push('.');
                name.push_str(extension);
            }
            let target = output_dir.join(name);
            match std::fs::write(&target, &damaged.data) {
                Ok(()) => true,
                Err(error) => {
                    eprintln!("cannot write {}: {error}", target.display());
                    false
                }
            }
        })
        .count()
}

fn collect_files(paths: &[PathBuf], recursive: bool) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for path in paths {
        if path.is_file() {
            files.push(path.clone());
        } else if path.is_dir() {
            let depth = if recursive { usize::MAX } else { 1 };
            for entry in WalkDir::new(path).max_depth(depth).into_iter().flatten() {
                if entry.file_type().is_file() {
                    files.push(entry.path());
                }
            }
        } else {
            eprintln!("skipping {}: not a file or directory", path.display());
        }
    }
    files.sort_unstable();
    files.dedup();
    files
}

fn configure_thread_pool(jobs: usize) {
    if jobs == 0 {
        return;
    }
    if let Err(error) = rayon::ThreadPoolBuilder::new().num_threads(jobs).build_global() {
        eprintln!("warning: could not set the thread count: {error}");
    }
}

/// The exit code reflects the least successful file, so a batch run fails loudly if any file
/// could not be fully recovered.
fn worst_outcome(reports: &[RepairReport]) -> Outcome {
    reports
        .iter()
        .map(effective_outcome)
        .max_by_key(|outcome| outcome.exit_code())
        .unwrap_or(Outcome::Healthy)
}

/// A repair that could not be written is a failed run even though the content came back, so
/// the exit code says so while the printed line still reports what was recovered.
fn effective_outcome(report: &RepairReport) -> Outcome {
    if report.winning_strategy.is_some() && report.output_path.is_none() && report.error.is_some() {
        return Outcome::Unrecoverable;
    }
    report.outcome
}

#[cfg(test)]
mod tests {
    use file_fixer_dev::file_fixer::mode::Mode;

    use super::*;

    fn report(outcome: Outcome) -> RepairReport {
        let mut report = RepairReport::failed(PathBuf::from("x"), Mode::Balanced, String::new());
        report.error = None;
        report.outcome = outcome;
        report
    }

    #[test]
    fn the_worst_outcome_drives_the_exit_code() {
        let reports = [
            report(Outcome::Healthy),
            report(Outcome::PartiallyRecovered),
            report(Outcome::FullyRepaired),
        ];
        assert_eq!(worst_outcome(&reports), Outcome::PartiallyRecovered);

        let reports = [report(Outcome::Healthy), report(Outcome::Unrecoverable)];
        assert_eq!(worst_outcome(&reports), Outcome::Unrecoverable);

        assert_eq!(worst_outcome(&[]), Outcome::Healthy);
    }

    #[test]
    fn an_all_healthy_batch_exits_successfully() {
        let reports = [report(Outcome::Healthy), report(Outcome::FullyRepaired)];
        assert_eq!(worst_outcome(&reports).exit_code(), 0);
    }
}
