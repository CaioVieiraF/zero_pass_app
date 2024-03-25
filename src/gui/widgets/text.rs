use iced::widget::text::{self, Appearance};

use super::{ZeroPassTheme, BACKGROUND, TEXT};

#[derive(Debug, Clone, Copy, Default)]
pub enum TextStyle {
    #[default]
    Light,
    Dark,
}

impl text::StyleSheet for ZeroPassTheme {
    type Style = TextStyle;

    fn appearance(&self, style: Self::Style) -> Appearance {
        match style {
            TextStyle::Dark => Appearance { color: Some(TEXT) },
            TextStyle::Light => Appearance {
                color: Some(BACKGROUND),
            },
        }
    }
}
