use iced::{overlay::menu, Background};

use super::{pick_list::PickListStyle, ZeroPassTheme, ACCENT, BACKGROUND, TEXT};

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
                text_color: TEXT,
                background: Background::Color(BACKGROUND),
                border: Default::default(),
                selected_text_color: TEXT,
                selected_background: Background::Color(ACCENT),
            },
        }
    }
}
