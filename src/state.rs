use std::time::Instant;

// mods
use crate::theme::{Theme, dark_theme};

// constants
pub const DEFAULT_WORK_DURATION: u16 = 25 * 60; // 25 min; in seconds
pub const DEFAULT_BREAK_DURATION: u16 = 5 * 60; // 5 min; in seconds

pub struct State {
    pub remaining_seconds: u16,
    pub starting_seconds: u16,
    pub started_at: Option<Instant>,
    pub is_running: bool,
    pub is_break: bool,
    pub work_duration: u16,
    pub break_duration: u16,
    pub theme: Theme,
}

pub fn initial_state() -> State {
    return State {
        remaining_seconds: DEFAULT_WORK_DURATION, // for only uses default; will add custom options later
        starting_seconds: 0,                      // abitrary
        started_at: None,
        is_running: false,
        is_break: false,
        work_duration: DEFAULT_WORK_DURATION,
        break_duration: DEFAULT_BREAK_DURATION,
        theme: dark_theme(),
    };
}
