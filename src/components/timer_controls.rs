use rune::{El, Family, TextStyle, Tx, el, row, text};
use std::time::Instant;

// mods
use crate::state::State;

use crate::timer::update_timer;

pub fn timer_controls(s: &State) -> El {
    // start button
    let start_button_text_content: &str = if s.is_running { "Pause" } else { "Start" };

    let start_button_text_style: TextStyle = TextStyle::new(15.0, 400.0).family(Family::SystemMono);

    let start_button_text: El = text(start_button_text_content)
        .color(s.theme.primary)
        .font(start_button_text_style);

    let start_button: El = row()
        .fill(s.theme.surface)
        .radius(6.0)
        .size(86.0, 36.0)
        .center()
        .cursor(el::Cursor::Pointer)
        .child(start_button_text)
        .on_press_tx(|s: &mut State, tx: &mut Tx| {
            if s.is_running {
                update_timer(s);
                s.started_at = None;
                s.is_running = false;
            } else if s.remaining_seconds > 0 {
                s.starting_seconds = s.remaining_seconds;
                s.started_at = Some(Instant::now());
                s.is_running = true;

                tx.request_poll();
            }
        });

    return start_button;
}
