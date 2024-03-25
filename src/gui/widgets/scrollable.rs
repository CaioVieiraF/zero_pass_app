use iced::{
    widget::{
        container,
        scrollable::{self, Appearance},
    },
    Background,
};

use super::{ZeroPassTheme, ACCENT, PRIMARY};

#[derive(Default)]
pub enum ScrollableStyle {
    #[default]
    Default,
}

impl scrollable::StyleSheet for ZeroPassTheme {
    type Style = ScrollableStyle;

    fn active(&self, _: &Self::Style) -> Appearance {
        Appearance {
            container: container::Appearance::default(),
            scrollbar: scrollable::Scrollbar {
                background: Some(Background::Color(ACCENT)),
                border: Default::default(),
                scroller: scrollable::Scroller {
                    color: PRIMARY,
                    border: Default::default(),
                },
            },
            gap: None,
        }
    }

    fn hovered(&self, _: &Self::Style, is_mouse_over_scrollbar: bool) -> Appearance {
        if is_mouse_over_scrollbar {
            Appearance {
                container: container::Appearance::default(),
                scrollbar: scrollable::Scrollbar {
                    background: Some(Background::Color(ACCENT)),
                    border: Default::default(),
                    scroller: scrollable::Scroller {
                        color: PRIMARY,
                        border: Default::default(),
                    },
                },
                gap: None,
            }
        } else {
            Appearance {
                container: container::Appearance::default(),
                scrollbar: scrollable::Scrollbar {
                    background: Some(Background::Color(ACCENT)),
                    border: Default::default(),
                    scroller: scrollable::Scroller {
                        color: PRIMARY,
                        border: Default::default(),
                    },
                },
                gap: None,
            }
        }
    }
}
