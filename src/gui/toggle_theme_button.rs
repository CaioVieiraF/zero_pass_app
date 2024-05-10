use crate::Message;

use super::widgets::{dark_icon, light_icon, Button, ZeroPassTheme};


pub fn toggle_theme_button<'a>(theme: &ZeroPassTheme) -> iced::Element<'a, Message, ZeroPassTheme> {
    let icon = match theme {
        ZeroPassTheme::Dark => dark_icon(),
        ZeroPassTheme::Light => light_icon(),
    };

    Button::new(icon).on_press(Message::ToggleTheme).into()
}
