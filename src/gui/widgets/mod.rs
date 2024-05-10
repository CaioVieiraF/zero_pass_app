pub mod button;
pub mod container;
mod menu;
mod pick_list;
mod scrollable;
mod slider;
pub mod text;
pub mod text_input;

use iced::{
    application::{self, Appearance},
    Color, Element, Font,
};

use crate::Message;

pub type TextInput<'a, Message> = iced::widget::TextInput<'a, Message, ZeroPassTheme>;
pub type Button<'a, Message> = iced::widget::Button<'a, Message, ZeroPassTheme>;
pub type Container<'a, Message> = iced::widget::Container<'a, Message, ZeroPassTheme>;
pub type Slider<'a, T, Message> = iced::widget::Slider<'a, T, Message, ZeroPassTheme>;
pub type Row<'a, Message> = iced::widget::Row<'a, Message, ZeroPassTheme>;
pub type Column<'a, Message> = iced::widget::Column<'a, Message, ZeroPassTheme>;
pub type Scrollable<'a, Message> = iced::widget::Scrollable<'a, Message, ZeroPassTheme>;
pub type PickList<'a, T, L, V, Message> =
    iced::widget::PickList<'a, T, L, V, Message, ZeroPassTheme>;
pub type Text<'a> = iced::widget::Text<'a, ZeroPassTheme>;

pub const BACKGROUND_DARK: Color = Color::from_rgb(0.0, 0.04, 0.06);
pub const BACKGROUND: Color = Color::from_rgb(0.94, 0.98, 1.0);
pub const TEXT_DARK: Color = Color::from_rgb(0.84, 0.96, 0.99);
pub const TEXT: Color = Color::from_rgb(0.004, 0.12, 0.16);
pub const PRIMARY_DARK: Color = Color::from_rgb(0.11, 0.7, 0.9);
pub const PRIMARY: Color = Color::from_rgb(0.1, 0.7, 0.9);
pub const SECONDARY_DARK: Color = Color::from_rgb(0.16, 0.33, 0.38);
pub const SECONDARY: Color = Color::from_rgb(0.62, 0.79, 0.84);
pub const ACCENT_DARK: Color = Color::from_rgb(0.64, 0.65, 0.09);
pub const ACCENT: Color = Color::from_rgb(0.9, 0.9, 0.35);

#[derive(Default, Clone)]
pub enum ZeroPassTheme {
    #[default]
    Dark,
    Light
}

#[derive(Default, Clone)]
pub enum ZeroPassStyle {
    #[default]
    Dark,
    Light,
}

impl application::StyleSheet for ZeroPassTheme {
    type Style = ZeroPassStyle;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match style {
            ZeroPassStyle::Dark => Appearance {
                background_color: BACKGROUND_DARK,
                text_color: TEXT_DARK,
            },
            ZeroPassStyle::Light => Appearance {
                background_color: BACKGROUND,
                text_color: TEXT,
            },
        }
    }
}

fn icon<'a>(code: char) -> Element<'a, Message, ZeroPassTheme> {
    const ICON_FONT: Font = Font::with_name("zero-pass-icons");

    iced::widget::text(code).font(ICON_FONT).into()
}

pub fn copy_icon<'a>() -> Element<'a, Message, ZeroPassTheme> {
    icon('\u{E800}')
}

pub fn light_icon<'a>() -> Element<'a, Message, ZeroPassTheme> {
    icon('\u{E801}')
}

pub fn dark_icon<'a>() -> Element<'a, Message, ZeroPassTheme> {
    icon('\u{E802}')
}
