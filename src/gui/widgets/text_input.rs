use iced::{
    border::Radius,
    widget::text_input::{self, Appearance},
    Background, Color,
};

use super::{ZeroPassTheme, ACCENT, BACKGROUND, PRIMARY, SECONDARY, TEXT};

#[derive(Default)]
pub enum TextInputStyle {
    #[default]
    Default,
}

impl text_input::StyleSheet for ZeroPassTheme {
    type Style = TextInputStyle;

    fn active(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(BACKGROUND),
            border: iced::Border {
                color: SECONDARY,
                width: 1.0,
                radius: Radius::from(4),
            },
            icon_color: PRIMARY,
        }
    }

    fn focused(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(BACKGROUND),
            border: iced::Border {
                color: ACCENT,
                width: 1.0,
                radius: Radius::from(4),
            },
            icon_color: PRIMARY,
        }
    }

    fn placeholder_color(&self, _: &Self::Style) -> Color {
        SECONDARY
    }

    fn value_color(&self, _: &Self::Style) -> Color {
        TEXT
    }

    fn disabled_color(&self, _: &Self::Style) -> Color {
        SECONDARY
    }

    fn selection_color(&self, _: &Self::Style) -> Color {
        TEXT
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        self.active(style)
    }
}
