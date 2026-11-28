use rune::{El, Family, TextStyle, text};

// mods
use crate::state::State;

pub fn timer_display(s: &State) -> El {
    // live values
    let minutes: u16 = s.remaining_seconds / 60;
    let seconds: u16 = s.remaining_seconds % 60;

    // timer text
    let timer_text_style: TextStyle = TextStyle::new(75.9, 500.0)
        .family(Family::SystemMono)
        .tracking(-4.125);

    let timer_text: El = text(format!("{:02}:{:02}", minutes, seconds))
        .color(s.theme.primary)
        .font(timer_text_style);

    return timer_text;
}
