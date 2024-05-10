use iced::{
    border::Radius,
    widget::text_input::{self, Appearance},
    Background, Color,
};

use super::{ZeroPassTheme, ACCENT_DARK, BACKGROUND_DARK, PRIMARY_DARK, SECONDARY_DARK, TEXT_DARK};

#[derive(Default)]
pub enum TextInputStyle {
    #[default]
    Default,
    Copy,
}

impl text_input::StyleSheet for ZeroPassTheme {
    type Style = TextInputStyle;

    fn active(&self, style: &Self::Style) -> Appearance {
        match style {
            TextInputStyle::Default => Appearance {
                background: Background::Color(BACKGROUND_DARK),
                border: iced::Border {
                    color: SECONDARY_DARK,
                    width: 1.0,
                    radius: Radius::from(4),
                },
                icon_color: PRIMARY_DARK,
            },
            TextInputStyle::Copy => Appearance {
                background: Background::Color(BACKGROUND_DARK),
                border: iced::Border {
                    color: ACCENT_DARK,
                    width: 1.0,
                    radius: Radius::from([4.0, 0.0, 0.0, 4.0]),
                },
                icon_color: PRIMARY_DARK,
            },
        }
    }

    fn focused(&self, style: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(BACKGROUND_DARK),
            border: iced::Border {
                color: ACCENT_DARK,
                width: 1.0,
                radius: match style {
                    TextInputStyle::Default => Radius::from(4),
                    TextInputStyle::Copy => Radius::from([4.0, 0.0, 0.0, 4.0]),
                },
            },
            icon_color: PRIMARY_DARK,
        }
    }

    fn placeholder_color(&self, _: &Self::Style) -> Color {
        SECONDARY_DARK
    }

    fn value_color(&self, _: &Self::Style) -> Color {
        TEXT_DARK
    }

    fn disabled_color(&self, _: &Self::Style) -> Color {
        SECONDARY_DARK
    }

    fn selection_color(&self, _: &Self::Style) -> Color {
        TEXT_DARK
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        self.active(style)
    }
}
