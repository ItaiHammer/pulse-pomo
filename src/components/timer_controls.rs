use rune::{El, Family, TextStyle, Tx, el, row, text};

// mods
use crate::state::State;
use crate::timer::{pause_timer, reset_timer, start_timer};

pub fn timer_controls(s: &State) -> El {
    let button_text_style: TextStyle = TextStyle::new(15.0, 400.0).family(Family::SystemMono);

    // start button
    let start_button_text_content: &str = if s.is_running { "Pause" } else { "Start" };

    let start_button_text: El = text(start_button_text_content)
        .color(s.theme.primary)
        .font(button_text_style);

    let start_button: El = row()
        .fill(s.theme.surface)
        .radius(6.0)
        .size(86.0, 36.0)
        .center()
        .cursor(el::Cursor::Pointer)
        .child(start_button_text)
        .on_press_tx(|s: &mut State, tx: &mut Tx| {
            if s.is_running {
                pause_timer(s);
            } else if s.remaining_seconds > 0 {
                start_timer(s);

                tx.request_poll();
            }
        });

    // reset button
    let reset_button_text: El = text("Reset").color(s.theme.primary).font(button_text_style);

    let reset_button: El = row()
        .fill(s.theme.surface)
        .radius(6.0)
        .size(86.0, 36.0)
        .center()
        .cursor(el::Cursor::Pointer)
        .child(reset_button_text)
        .on_press(|s: &mut State| {
            reset_timer(s);
        });

    return row()
        .center()
        .gap(8.0)
        .child(start_button)
        .child(reset_button);
}
