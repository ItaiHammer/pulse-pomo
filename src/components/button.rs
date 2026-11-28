use rune::{El, Family, Icon, TextStyle, el, icon, row, text};

// mods
use crate::icons::DEFAULT_ICON_SIZE;
use crate::theme::Theme;

fn base_button(t: &Theme) -> El {
    let button_element: El = row()
        .fill(t.surface)
        .radius(6.0)
        .h(36.0)
        .min_w(36.0)
        .padding(8.0)
        .gap(4.0)
        .center()
        .cursor(el::Cursor::Pointer);

    return button_element;
}

pub fn text_button(t: &Theme, label: &str, i: Option<&'static Icon>) -> El {
    let style: TextStyle = TextStyle::new(15.0, 400.0).family(Family::SystemMono);

    let mut button_element: El = base_button(t);

    // if icon exists, add it to the button
    if let Some(value) = i {
        let icon_element: El = icon(&value, DEFAULT_ICON_SIZE).color(t.primary);
        button_element = button_element.child(icon_element);
    }

    // adding text
    let text_element: El = text(label).color(t.primary).font(style);
    button_element = button_element.child(text_element);

    return button_element;
}

pub fn icon_button(t: &Theme, i: &'static Icon) -> El {
    let mut button_element: El = base_button(t);

    // adding icon
    let icon_element: El = icon(&i, DEFAULT_ICON_SIZE).color(t.primary);
    button_element = button_element.child(icon_element);

    return button_element;
}
