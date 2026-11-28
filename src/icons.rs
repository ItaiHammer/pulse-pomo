use rune::Icon;

pub const DEFAULT_ICON_VIEW_SIZE: f32 = 24.0;
pub const DEFAULT_ICON_SIZE: f32 = 16.0;

// play icon
const PLAY_SVG: &str = include_str!("./assets/icons/play.svg");
pub static PLAY_ICON: Icon = Icon {
    name: "play",
    view: DEFAULT_ICON_VIEW_SIZE,
    body: PLAY_SVG,
};

// pause icon
const PAUSE_SVG: &str = include_str!("./assets/icons/pause.svg");
pub static PAUSE_ICON: Icon = Icon {
    name: "pause",
    view: DEFAULT_ICON_VIEW_SIZE,
    body: PAUSE_SVG,
};

// skip icon
const SKIP_SVG: &str = include_str!("./assets/icons/skip.svg");
pub static SKIP_ICON: Icon = Icon {
    name: "skip",
    view: DEFAULT_ICON_VIEW_SIZE,
    body: SKIP_SVG,
};

// reset icon
const RESET_SVG: &str = include_str!("./assets/icons/reset.svg");
pub static RESET_ICON: Icon = Icon {
    name: "reset",
    view: DEFAULT_ICON_VIEW_SIZE,
    body: RESET_SVG,
};
