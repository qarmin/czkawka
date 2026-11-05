//! Repairs damaged media files, recovering as much content as the data allows.
//!
//! The core is not a fixed repair pipeline but a search: each format proposes a set of
//! strategies, every strategy produces a candidate output, candidates are scored, and the
//! best one wins. That shape exists because "repair" is genuinely ambiguous for a damaged
//! file - cutting at the last verifiable point and resynchronizing past the damage are both
//! defensible, and which one is better depends on the file.

pub mod cancel;
pub mod candidate;
pub mod corrupt;
pub mod engine;
pub mod error;
pub mod format;
pub mod formats;
pub mod io;
pub mod mode;
pub mod output;
pub mod report;

pub use candidate::{Candidate, Fidelity, Score, Selection, SelectionReason, select_best};
pub use engine::{Budget, RepairOptions, RepairResult, repair_data, repair_file};
pub use error::{FixerError, Result};
pub use format::{Detection, DetectionSource, Format};
pub use mode::{Capabilities, CapabilityOverrides, Mode};
pub use report::{Outcome, RepairReport};
