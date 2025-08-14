use std::io::Write;
use std::time::Duration;

use humansize::{BINARY, format_size};

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
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

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone)]
pub struct Config {
    pub name: &'static str,
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
}

impl Config {
    pub fn to_str(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
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
            "LTO: {}, Debug: {}, Opt: {}, Build/Check: {}, Codegen Units: {}, Panic: {}, Split Debug: {}, Overflow Checks: {}, Incremental: {}",
            self.lto.to_str(),
            self.debug.to_str(),
            self.opt_level.to_str(),
            self.build_or_check.to_str(),
            self.codegen_units.to_str(),
            self.panic.to_str(),
            self.split_debug.to_str(),
            self.overflow_checks.to_str(),
            self.incremental.to_str()
        )
    }
}

pub struct Results {
    pub output_file_size: u64,
    pub target_folder_size: u64,
    pub compilation_time: Duration,
    pub config: Config,
    pub threads_number: u32,
    pub project: String,
    pub cranelift: bool,
    pub use_mold: bool,
    pub rebuild_time: Duration,
}

impl Results {
    pub fn write_header_to_file(file_writer: &mut std::fs::File) -> std::io::Result<()> {
        writeln!(
            file_writer,
            "Config|Output File Size|Output File Size(in bytes)|Target Folder Size|Target Folder Size(in bytes)|Compilation Time(seconds)|Compilation Time|Threads|Rebuild Time(seconds)|Rebuild Time",
        )?;
        Ok(())
    }
    pub fn save_to_file(&self, file_writer: &mut std::fs::File) -> std::io::Result<()> {
        let cranelift = if self.cranelift { "+ cranelift" } else { "+ llvm" };

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

        let linker = if self.use_mold { "+ mold" } else { "+ ld" };

        writeln!(
            file_writer,
            "{} {} {} __ {}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.config.name,
            cranelift,
            linker,
            self.project,
            file_size_pretty,
            file_size_number,
            format_size(self.target_folder_size, BINARY),
            self.target_folder_size,
            self.compilation_time.as_secs_f32(),
            duration_to_pretty_time(self.compilation_time),
            self.threads_number,
            self.rebuild_time.as_secs_f32(),
            duration_to_pretty_time(self.rebuild_time)
        )?;
        Ok(())
    }
}

fn duration_to_pretty_time(duration: std::time::Duration) -> String {
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
