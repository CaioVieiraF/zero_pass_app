use iced::{overlay::menu, Background};

use super::{pick_list::PickListStyle, ZeroPassTheme, ACCENT_DARK, BACKGROUND_DARK, TEXT_DARK};

#[derive(Default, Clone)]
pub enum MenuStyle {
    #[default]
    Default,
}

impl From<PickListStyle> for MenuStyle {
    fn from(value: PickListStyle) -> Self {
        match value {
            PickListStyle::Default => MenuStyle::Default,
        }
    }
}

impl menu::StyleSheet for ZeroPassTheme {
    type Style = MenuStyle;

    fn appearance(&self, style: &Self::Style) -> menu::Appearance {
        match style {
            MenuStyle::Default => menu::Appearance {
                text_color: TEXT_DARK,
                background: Background::Color(BACKGROUND_DARK),
                border: Default::default(),
                selected_text_color: TEXT_DARK,
                selected_background: Background::Color(ACCENT_DARK),
            },
        }
    }
}
