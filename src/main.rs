use rune::{AppSpec, Cx, El, Family, TextStyle, Tx, col, el, row, text};

// mods
mod theme;

mod state;
use state::{State, initial_state};

mod timer;
use timer::poll;

mod icons;

mod components;
use components::{timer_controls, timer_display};

// app
fn main() {
    let mut app = AppSpec::new("Pulse Pomodoro", (400.0, 600.0), initial_state, view);

    app.poll = Some(poll);

    rune::run(app)
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let timer_display: El = timer_display(s);
    let timer_controls: El = timer_controls(s);

    // root
    let root: El = col()
        .w_full()
        .h_full()
        .center()
        .gap(16.0)
        .child(timer_display)
        .child(timer_controls)
        .fill(s.theme.background);

    return root;
}
