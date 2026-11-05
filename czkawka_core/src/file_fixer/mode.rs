use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// The four independent things a repair is allowed to do. Presets are sugar over these;
/// strategies ask the capability set what they may attempt, never the preset name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Capabilities {
    /// Drop data that cannot be validated (cut a file short, discard trailing frames).
    pub truncate: bool,
    /// Re-encode content, losing generational quality, to reach a shape remuxing cannot.
    pub reencode: bool,
    /// Reconstruct structures that are missing entirely (`moov`, `IHDR`, MCU resynchronization).
    pub guess: bool,
    /// Emit synthetic content in place of unrecoverable data (grey blocks, repeated frames).
    pub fill_gaps: bool,
}

impl Capabilities {
    pub const NONE: Self = Self {
        truncate: false,
        reencode: false,
        guess: false,
        fill_gaps: false,
    };

    pub const ALL: Self = Self {
        truncate: true,
        reencode: true,
        guess: true,
        fill_gaps: true,
    };

    /// True when the capability set can only produce byte-exact subsets of the input.
    pub fn is_lossless_only(self) -> bool {
        !self.reencode && !self.guess && !self.fill_gaps
    }

    pub fn is_empty(self) -> bool {
        self == Self::NONE
    }

    /// What `self` asks for that `available` does not grant.
    pub fn missing_from(self, available: Self) -> Self {
        Self {
            truncate: self.truncate && !available.truncate,
            reencode: self.reencode && !available.reencode,
            guess: self.guess && !available.guess,
            fill_gaps: self.fill_gaps && !available.fill_gaps,
        }
    }

    /// Packed form, so the search can record declined capabilities in an atomic.
    pub const fn bits(self) -> u8 {
        (self.truncate as u8) | (self.reencode as u8) << 1 | (self.guess as u8) << 2 | (self.fill_gaps as u8) << 3
    }

    pub const fn from_bits(bits: u8) -> Self {
        Self {
            truncate: bits & 1 != 0,
            reencode: bits & 2 != 0,
            guess: bits & 4 != 0,
            fill_gaps: bits & 8 != 0,
        }
    }

    pub fn from_names(names: &[String]) -> Self {
        let has = |wanted: &str| names.iter().any(|name| name == wanted);
        Self {
            truncate: has("truncate"),
            reencode: has("reencode"),
            guess: has("guess"),
            fill_gaps: has("fill-gaps"),
        }
    }

    /// Names in the spelling `--allow` expects.
    pub fn names(self) -> Vec<&'static str> {
        let mut names = Vec::new();
        for (present, name) in [
            (self.truncate, "truncate"),
            (self.reencode, "reencode"),
            (self.guess, "guess"),
            (self.fill_gaps, "fill-gaps"),
        ] {
            if present {
                names.push(name);
            }
        }
        names
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Structural repairs only. Never discards data, never invents any.
    Safe,
    /// Adds truncation to the last verifiably good point.
    #[default]
    Balanced,
    /// Everything: re-encoding, reconstruction of missing structures, gap filling.
    Aggressive,
}

impl Mode {
    pub fn capabilities(self) -> Capabilities {
        match self {
            Self::Safe => Capabilities::NONE,
            Self::Balanced => Capabilities {
                truncate: true,
                ..Capabilities::NONE
            },
            Self::Aggressive => Capabilities::ALL,
        }
    }

    pub const ALL: [Self; 3] = [Self::Safe, Self::Balanced, Self::Aggressive];

    /// The least permissive preset that grants everything in `wanted`.
    ///
    /// Used to turn "this file had a candidate the mode forbade" into advice the user can act
    /// on, rather than an `unrecoverable` that hides the fact a repair existed.
    pub fn smallest_granting(wanted: Capabilities) -> Option<Self> {
        Self::ALL.into_iter().find(|mode| wanted.missing_from(mode.capabilities()).is_empty())
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Safe => "safe",
            Self::Balanced => "balanced",
            Self::Aggressive => "aggressive",
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "safe" => Ok(Self::Safe),
            "balanced" => Ok(Self::Balanced),
            "aggressive" => Ok(Self::Aggressive),
            other => Err(format!("unknown mode {other:?}, expected safe, balanced or aggressive")),
        }
    }
}

/// Per-capability overrides layered on top of a preset. `None` means "leave the preset alone".
#[derive(Debug, Clone, Copy, Default)]
pub struct CapabilityOverrides {
    pub truncate: Option<bool>,
    pub reencode: Option<bool>,
    pub guess: Option<bool>,
    pub fill_gaps: Option<bool>,
}

impl CapabilityOverrides {
    pub fn apply_to(self, base: Capabilities) -> Capabilities {
        Capabilities {
            truncate: self.truncate.unwrap_or(base.truncate),
            reencode: self.reencode.unwrap_or(base.reencode),
            guess: self.guess.unwrap_or(base.guess),
            fill_gaps: self.fill_gaps.unwrap_or(base.fill_gaps),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_mode_grants_nothing() {
        assert_eq!(Mode::Safe.capabilities(), Capabilities::NONE);
        assert!(Mode::Safe.capabilities().is_lossless_only());
    }

    #[test]
    fn balanced_may_truncate_but_stays_lossless() {
        let caps = Mode::Balanced.capabilities();
        assert!(caps.truncate);
        assert!(caps.is_lossless_only());
    }

    #[test]
    fn aggressive_grants_everything() {
        assert_eq!(Mode::Aggressive.capabilities(), Capabilities::ALL);
        assert!(!Mode::Aggressive.capabilities().is_lossless_only());
    }

    #[test]
    fn overrides_layer_on_top_of_preset() {
        let overrides = CapabilityOverrides {
            truncate: Some(true),
            ..CapabilityOverrides::default()
        };
        let caps = overrides.apply_to(Mode::Safe.capabilities());
        assert!(caps.truncate);
        assert!(!caps.reencode);
        assert!(caps.is_lossless_only());
    }

    #[test]
    fn capabilities_survive_the_packed_form() {
        for bits in 0..16u8 {
            let caps = Capabilities::from_bits(bits);
            assert_eq!(caps.bits(), bits);
            assert_eq!(Capabilities::from_names(&caps.names().iter().map(|n| n.to_string()).collect::<Vec<_>>()), caps);
        }
        assert!(Capabilities::NONE.is_empty());
        assert!(!Capabilities::ALL.is_empty());
    }

    #[test]
    fn only_the_capabilities_actually_absent_are_reported_missing() {
        let wanted = Capabilities {
            truncate: true,
            guess: true,
            ..Capabilities::NONE
        };
        let missing = wanted.missing_from(Mode::Balanced.capabilities());
        assert_eq!(missing.names(), vec!["guess"]);
        assert!(wanted.missing_from(Capabilities::ALL).is_empty());
    }

    /// The advice must name the cheapest mode that works, not simply the most permissive one.
    #[test]
    fn the_suggested_mode_is_the_least_permissive_one_that_works() {
        let truncate = Capabilities {
            truncate: true,
            ..Capabilities::NONE
        };
        assert_eq!(Mode::smallest_granting(Capabilities::NONE), Some(Mode::Safe));
        assert_eq!(Mode::smallest_granting(truncate), Some(Mode::Balanced));
        assert_eq!(Mode::smallest_granting(Capabilities::ALL), Some(Mode::Aggressive));
    }

    #[test]
    fn mode_round_trips_through_string() {
        for mode in Mode::ALL {
            assert_eq!(mode.as_str().parse::<Mode>().unwrap(), mode);
        }
        "wild".parse::<Mode>().unwrap_err();
    }
}
