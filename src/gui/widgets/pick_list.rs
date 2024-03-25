use iced::{widget::pick_list, Background};

use super::{ZeroPassTheme, BACKGROUND, SECONDARY, TEXT};

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
                background: Background::Color(BACKGROUND),
                text_color: TEXT,
                placeholder_color: SECONDARY,
                handle_color: SECONDARY,
                border: Default::default(),
            },
        }
    }

    fn hovered(&self, style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        match style {
            PickListStyle::Default => pick_list::Appearance {
                background: Background::Color(TEXT),
                text_color: BACKGROUND,
                placeholder_color: SECONDARY,
                handle_color: SECONDARY,
                border: Default::default(),
            },
        }
    }
}
