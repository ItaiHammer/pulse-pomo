use rune::Tx;
use std::time::{Duration, Instant};

// mods
use crate::state::State;

pub fn poll(s: &mut State, _: f64, tx: &mut Tx) -> bool {
    let old = s.remaining_seconds;

    update_timer(s);

    if s.is_running {
        tx.wake_after(Duration::from_millis(250));
    }

    return old != s.remaining_seconds;
}

pub fn update_timer(s: &mut State) {
    // updates the timer state variable

    if let Some(inst) = s.started_at {
        let elapsed: u64 = inst.elapsed().as_secs();
        let remaining: u64 = (s.starting_seconds as u64).saturating_sub(elapsed);

        s.remaining_seconds = remaining as u16;

        if (s.remaining_seconds == 0) {
            // ran out of time
            s.is_running = false;
            s.started_at = None;
        }
    }
}

pub fn start_timer(s: &mut State) {
    s.starting_seconds = s.remaining_seconds;
    s.started_at = Some(Instant::now());
    s.is_running = true;
}

pub fn pause_timer(s: &mut State) {
    update_timer(s);
    s.started_at = None;
    s.is_running = false;
}

pub fn reset_timer(s: &mut State) {
    let duration = if s.is_break {
        s.break_duration
    } else {
        s.work_duration
    };

    s.remaining_seconds = duration;
    s.starting_seconds = duration;

    s.started_at = if s.is_running {
        Some(Instant::now())
    } else {
        None
    };
}

pub fn add_minute(s: &mut State) {
    let seconds_to_add: u16 = ((60 * 60) - s.remaining_seconds).min(60); // can't go more than an hour (60 secs * 60 min)
    s.remaining_seconds += seconds_to_add;
    s.starting_seconds += seconds_to_add;
}
