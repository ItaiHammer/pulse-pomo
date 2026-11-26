use rune::{AppSpec, Cx, El, Family, TextStyle, Tx, col, el, row, text};
use std::time::{Duration, Instant};

mod theme;
use crate::theme::{Theme, dark_theme};

// state
struct State {
    remaining_seconds: u16,
    starting_seconds: u16,
    started_at: Option<Instant>,
    is_running: bool,
    is_break: bool,
    theme: Theme,
}

fn initial_state() -> State {
    return State {
        remaining_seconds: 1500, // 25 min * 60 sec/min = 1500 sec
        starting_seconds: 0,     // abitrary
        started_at: None,
        is_running: false,
        is_break: false,
        theme: dark_theme(),
    };
}

// app
fn main() {
    let mut app = AppSpec::new("Pulse Pomodoro", (400.0, 600.0), initial_state, view);

    app.poll = Some(poll);

    rune::run(app)
}

fn view(s: &State, _: &mut Cx<State>) -> El {
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

    // root
    let root: El = col()
        .w_full()
        .h_full()
        .center()
        .gap(16.0)
        .child(timer_text)
        .child(start_button)
        .fill(s.theme.background);

    return root;
}

fn poll(s: &mut State, _: f64, tx: &mut Tx) -> bool {
    let old = s.remaining_seconds;

    update_timer(s);

    if s.is_running {
        tx.wake_after(Duration::from_millis(250));
    }

    old != s.remaining_seconds
}

fn update_timer(s: &mut State) {
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
