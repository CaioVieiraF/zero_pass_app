use iced::widget::text::{self, Appearance};

use super::{ZeroPassTheme, BACKGROUND_DARK, TEXT_DARK};

#[derive(Debug, Clone, Copy, Default)]
pub enum TextStyle {
    #[default]
    Dark,
    Light,
}

impl text::StyleSheet for ZeroPassTheme {
    type Style = TextStyle;

    fn appearance(&self, style: Self::Style) -> Appearance {
        match style {
            TextStyle::Dark => Appearance { color: Some(TEXT_DARK) },
            TextStyle::Light => Appearance {
                color: Some(BACKGROUND_DARK),
            },
        }
    }
}
