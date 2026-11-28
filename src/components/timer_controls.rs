use rune::{El, Icon, Tx, row};

// mods
use crate::components::button::{icon_button, text_button};
use crate::icons::{PAUSE_ICON, PLAY_ICON, RESET_ICON, SKIP_ICON};
use crate::state::State;
use crate::timer::{add_minute, pause_timer, reset_timer, start_timer};

pub fn timer_controls(s: &State) -> El {
    // +1 min button
    let add_minute_button = text_button(&s.theme, "+1m", None).on_press(|s: &mut State| {
        add_minute(s);
    });

    // start/pause button
    let toggle_icon: &'static Icon = if s.is_running {
        &PAUSE_ICON
    } else {
        &PLAY_ICON
    };

    let toggle_label = if s.is_running { "Pause" } else { "Start" };

    let toggle_button = text_button(&s.theme, toggle_label, Some(toggle_icon)).on_press_tx(
        |s: &mut State, tx: &mut Tx| {
            if s.is_running {
                pause_timer(s);
            } else if s.remaining_seconds > 0 {
                start_timer(s);
                tx.request_poll();
            }
        },
    );

    // reset button
    let reset_button = icon_button(&s.theme, &RESET_ICON).on_press(|s: &mut State| {
        reset_timer(s);
    });

    // skip button
    let skip_button = icon_button(&s.theme, &SKIP_ICON);

    // Controls container
    row()
        .center()
        .gap(8.0)
        .child(add_minute_button)
        .child(toggle_button)
        .child(reset_button)
        .child(skip_button)
}
