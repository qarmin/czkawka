use std::io::Write;
use std::time::Duration;

use humansize::{BINARY, format_size};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub project: Project,
    pub build_config: Vec<BuildConfigRead>,
    #[serde(skip)]
    pub build_config_converted: Vec<BuildConfig>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path_to_main_rs_file: String,
    pub path_to_clean_with_git: String,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildConfigRead {
    pub name: String,
    pub rust_base_config: RustBaseConfig,
    pub lto: Option<Lto>,
    pub debug: Option<Debugg>,
    pub opt_level: Option<OptLevel>,
    pub build_or_check: Option<BuildOrCheck>,
    pub codegen_units: Option<CodegenUnits>,
    pub panic: Option<Panic>,
    pub split_debug: Option<SplitDebug>,
    pub overflow_checks: Option<OverflowChecks>,
    pub incremental: Option<Incremental>,
    pub build_std: Option<bool>,
    pub native: Option<bool>,
    pub cranelift: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildConfig {
    pub name: String,
    pub rust_base_config: RustBaseConfig,
    pub lto: Lto,
    pub debug: Debugg,
    pub opt_level: OptLevel,
    pub build_or_check: BuildOrCheck,
    pub codegen_units: CodegenUnits,
    pub panic: Panic,
    pub split_debug: SplitDebug,
    pub overflow_checks: OverflowChecks,
    pub incremental: Incremental,
    pub build_std: bool,
    pub native: bool,
    pub cranelift: bool,
}

impl From<BuildConfigRead> for BuildConfig {
    fn from(config: BuildConfigRead) -> Self {
        let base_config = match config.rust_base_config {
            RustBaseConfig::Release => BuildConfig {
                name:"release".to_string(),
                rust_base_config: RustBaseConfig::Release,
                lto: Lto::Off,
                debug: Debugg::None,
                opt_level: OptLevel::Three,
                build_or_check: BuildOrCheck::Build,
                codegen_units: CodegenUnits::Sixteen,
                panic: Panic::Unwind,
                split_debug: SplitDebug::Off,
                overflow_checks: OverflowChecks::Off,
                incremental: Incremental::Off,
                build_std: false,
                native: false,
                cranelift: false
            },
            RustBaseConfig::Debug => BuildConfig {
                name: "debug".to_string(),
                rust_base_config: RustBaseConfig::Debug,
                lto: Lto::Off,
                debug: Debugg::Full,
                opt_level: OptLevel::Zero,
                build_or_check: BuildOrCheck::Build,
                codegen_units: CodegenUnits::Default,
                panic: Panic::Unwind,
                split_debug: SplitDebug::Off,
                overflow_checks: OverflowChecks::On,
                incremental: Incremental::On,
                build_std: false,
                native: false,
                cranelift: false
            }
        };

        Self {
            name: config.name,
            rust_base_config: base_config.rust_base_config,
            lto: config.lto.unwrap_or(base_config.lto),
            debug: config.debug.unwrap_or(base_config.debug),
            opt_level: config.opt_level.unwrap_or(base_config.opt_level),
            build_or_check: config.build_or_check.unwrap_or(base_config.build_or_check),
            codegen_units: config.codegen_units.unwrap_or(base_config.codegen_units),
            panic: config.panic.unwrap_or(base_config.panic),
            split_debug: config.split_debug.unwrap_or(base_config.split_debug),
            overflow_checks: config.overflow_checks.unwrap_or(base_config.overflow_checks),
            incremental: config.incremental.unwrap_or(base_config.incremental),
            build_std: config.build_std.unwrap_or(false),
            native: config.native.unwrap_or(false),
            cranelift: config.cranelift.unwrap_or(false),
        }
    }
}

impl BuildConfig {
    pub fn to_str(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            self.rust_base_config.to_str(),
            self.lto.to_str(),
            self.debug.to_str(),
            self.opt_level.to_str(),
            self.codegen_units.to_str(),
            self.panic.to_str(),
            self.split_debug.to_str(),
            self.overflow_checks.to_str(),
            self.incremental.to_str(),
        )
    }
    pub fn to_string_short(&self) -> String {
        format!(
            "LTO: {}, Debug: {}, Opt: {}, Build/Check: {}, Codegen Units: {}, Panic: {}, Split Debug: {}, Overflow Checks: {}, Incremental: {}, Build Std: {}, Cranelift: {}",
            self.lto.to_str(),
            self.debug.to_str(),
            self.opt_level.to_str(),
            self.build_or_check.to_str(),
            self.codegen_units.to_str(),
            self.panic.to_str(),
            self.split_debug.to_str(),
            self.overflow_checks.to_str(),
            self.incremental.to_str(),
            self.build_std,
            self.cranelift
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RustBaseConfig {
    Release,
    Debug
}

impl RustBaseConfig {
    pub fn to_str(&self) -> &'static str {
        match self {
            RustBaseConfig::Release => "inherits=\"release\"",
            RustBaseConfig::Debug => "inherits=\"dev\"",
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq,  Serialize, Deserialize)]
pub enum Debugg {
    None,
    LineDirectivesOnly,
    LineTablesOnly,
    Limited,
    Full,
}

impl Debugg {
    fn to_str(self) -> &'static str {
        match self {
            Debugg::None => "debug=\"none\"",
            Debugg::LineDirectivesOnly => "debug=\"line-directives-only\"",
            Debugg::LineTablesOnly => "debug=\"line-tables-only\"",
            Debugg::Limited => "debug=1",
            Debugg::Full => "debug=2",
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq,  Serialize, Deserialize)]
pub enum SplitDebug {
    Off,
    Packed,
    Unpacked,
}
impl SplitDebug {
    fn to_str(self) -> &'static str {
        match self {
            SplitDebug::Off => "split-debuginfo=\"off\"",
            SplitDebug::Packed => "split-debuginfo=\"packed\"",
            SplitDebug::Unpacked => "split-debuginfo=\"unpacked\"",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq,  Serialize, Deserialize)]
pub enum OptLevel {
    Zero,
    One,
    Two,
    Three,
    S,
}
impl OptLevel {
    fn to_str(self) -> &'static str {
        match self {
            OptLevel::Zero => "opt-level=0",
            OptLevel::One => "opt-level=1",
            OptLevel::Two => "opt-level=2",
            OptLevel::Three => "opt-level=3",
            OptLevel::S => "opt-level=\"s\"",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq,  Serialize, Deserialize)]
pub enum Lto {
    Off,
    Thin,
    Fat,
}

impl Lto {
    fn to_str(self) -> &'static str {
        match self {
            Lto::Off => "lto=\"off\"",
            Lto::Thin => "lto=\"thin\"",
            Lto::Fat => "lto=\"fat\"",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq,  Serialize, Deserialize)]
pub enum BuildOrCheck {
    Build,
    Check,
}

impl BuildOrCheck {
    fn to_str(self) -> &'static str {
        match self {
            BuildOrCheck::Build => "build",
            BuildOrCheck::Check => "check",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq,  Serialize, Deserialize)]
pub enum CodegenUnits {
    One,
    Sixteen,
    Default,
}
impl CodegenUnits {
    fn to_str(self) -> &'static str {
        match self {
            CodegenUnits::One => "codegen-units=1",
            CodegenUnits::Sixteen => "codegen-units=16",
            CodegenUnits::Default => "codegen-units=256",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq,  Serialize, Deserialize)]
pub enum Panic {
    Unwind,
    Abort,
}
impl Panic {
    fn to_str(self) -> &'static str {
        match self {
            Panic::Unwind => "panic=\"unwind\"",
            Panic::Abort => "panic=\"abort\"",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq,  Serialize, Deserialize)]
pub enum OverflowChecks {
    On,
    Off,
}
impl OverflowChecks {
    fn to_str(self) -> &'static str {
        match self {
            OverflowChecks::On => "overflow-checks=true",
            OverflowChecks::Off => "overflow-checks=false",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq,  Serialize, Deserialize)]
pub enum Incremental {
    On,
    Off,
}
impl Incremental {
    fn to_str(self) -> &'static str {
        match self {
            Incremental::On => "incremental=true",
            Incremental::Off => "incremental=false",
        }
    }
}



pub struct Results {
    pub output_file_size: u64,
    pub target_folder_size: u64,
    pub compilation_time: Duration,
    pub build_config: BuildConfig,
    pub project: Project,
    pub rebuild_time: Duration,
}

impl Results {
    pub fn write_header_to_file(file_writer: &mut std::fs::File) -> std::io::Result<()> {
        writeln!(
            file_writer,
            "BuildConfig|Output File Size|Output File Size(in bytes)|Target Folder Size|Target Folder Size(in bytes)|Compilation Time(seconds)|Compilation Time|Rebuild Time(seconds)|Rebuild Time",
        )?;
        Ok(())
    }
    pub fn save_to_file(&self, file_writer: &mut std::fs::File) -> std::io::Result<()> {
        let file_size_pretty = if self.output_file_size == 0 {
            "-".to_string()
        } else {
            format_size(self.output_file_size, BINARY)
        };
        let file_size_number = if self.output_file_size == 0 {
            "-".to_string()
        } else {
            self.output_file_size.to_string()
        };

        writeln!(
            file_writer,
            "{} __ {}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.build_config.name,
            self.project.name,
            file_size_pretty,
            file_size_number,
            format_size(self.target_folder_size, BINARY),
            self.target_folder_size,
            self.compilation_time.as_secs_f32(),
            duration_to_pretty_time(self.compilation_time),
            self.rebuild_time.as_secs_f32(),
            duration_to_pretty_time(self.rebuild_time)
        )?;
        Ok(())
    }
}

fn duration_to_pretty_time(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;

    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours % 24, minutes % 60, seconds % 60)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes % 60, seconds % 60)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds % 60)
    } else {
        format!("{}s", seconds)
    }
}
