use std::time::Instant;

// mods
use crate::theme::{Theme, dark_theme};

pub struct State {
    pub remaining_seconds: u16,
    pub starting_seconds: u16,
    pub started_at: Option<Instant>,
    pub is_running: bool,
    pub is_break: bool,
    pub theme: Theme,
}

pub fn initial_state() -> State {
    return State {
        remaining_seconds: 1500, // 25 min * 60 sec/min = 1500 sec
        starting_seconds: 0,     // abitrary
        started_at: None,
        is_running: false,
        is_break: false,
        theme: dark_theme(),
    };
}
