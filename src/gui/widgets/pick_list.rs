use iced::{widget::pick_list, Background};

use super::{ZeroPassTheme, BACKGROUND_DARK, SECONDARY_DARK, TEXT_DARK};

#[derive(Default, Clone)]
pub enum PickListStyle {
    #[default]
    Default,
}

impl pick_list::StyleSheet for ZeroPassTheme {
    type Style = PickListStyle;

    fn active(&self, style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        match style {
            PickListStyle::Default => pick_list::Appearance {
                background: Background::Color(BACKGROUND_DARK),
                text_color: TEXT_DARK,
                placeholder_color: SECONDARY_DARK,
                handle_color: SECONDARY_DARK,
                border: Default::default(),
            },
        }
    }

    fn hovered(&self, style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        match style {
            PickListStyle::Default => pick_list::Appearance {
                background: Background::Color(TEXT_DARK),
                text_color: BACKGROUND_DARK,
                placeholder_color: SECONDARY_DARK,
                handle_color: SECONDARY_DARK,
                border: Default::default(),
            },
        }
    }
}
