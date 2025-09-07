use std::cell::RefCell;
use std::rc::Rc;

pub const KEY_DELETE: u32 = 119;
pub const KEY_ENTER: u32 = 36;
pub const KEY_SPACE: u32 = 65;

pub const MAIN_ROW_COLOR: &str = "#222222";
pub const HEADER_ROW_COLOR: &str = "#111111";
pub const TEXT_COLOR: &str = "#ffffff";

pub type SharedState<T> = Rc<RefCell<Option<T>>>;
