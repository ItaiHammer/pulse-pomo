use rune::Tx;
use std::time::Duration;

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
