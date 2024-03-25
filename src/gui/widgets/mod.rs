pub mod button;
pub mod container;
mod menu;
mod pick_list;
mod scrollable;
mod slider;
pub mod text;
mod text_input;

use iced::{
    application::{self, Appearance},
    Color,
};

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

pub const BACKGROUND: Color = Color::from_rgb(0.0, 0.04, 0.06);
pub const TEXT: Color = Color::from_rgb(0.84, 0.96, 0.99);
pub const PRIMARY: Color = Color::from_rgb(0.11, 0.7, 0.9);
pub const SECONDARY: Color = Color::from_rgb(0.16, 0.33, 0.38);
pub const ACCENT: Color = Color::from_rgb(0.64, 0.65, 0.09);

#[derive(Default, Clone)]
pub enum ZeroPassTheme {
    #[default]
    Dark,
}

#[derive(Default)]
pub enum ZeroPassStyle {
    #[default]
    Default,
}

impl application::StyleSheet for ZeroPassTheme {
    type Style = ZeroPassStyle;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background_color: BACKGROUND,
            text_color: TEXT,
        }
    }
}
