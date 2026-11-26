use rune::Color;

pub struct Theme {
    pub background: Color,
    pub surface: Color,
    pub muted: Color,
    pub secondary: Color,
    pub primary: Color,
}

pub fn dark_theme() -> Theme {
    return Theme {
        background: rune::color::hex("#101010").unwrap(),
        surface: rune::color::hex("#242424").unwrap(),
        muted: rune::color::hex("#333333").unwrap(),
        secondary: rune::color::hex("#888888").unwrap(),
        primary: rune::color::hex("#E3E3E3").unwrap(),
    };
}
