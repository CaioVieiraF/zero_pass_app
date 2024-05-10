use iced::widget::slider;

use super::{ZeroPassTheme, ACCENT_DARK, BACKGROUND_DARK, PRIMARY_DARK, SECONDARY_DARK};

#[derive(Default)]
pub enum SliderStyle {
    #[default]
    Default,
}

impl slider::StyleSheet for ZeroPassTheme {
    type Style = SliderStyle;

    fn active(&self, style: &Self::Style) -> slider::Appearance {
        match style {
            SliderStyle::Default => slider::Appearance {
                rail: slider::Rail {
                    colors: (SECONDARY_DARK, BACKGROUND_DARK),
                    width: 1.0,
                    border_radius: Default::default(),
                },
                handle: slider::Handle {
                    shape: slider::HandleShape::Rectangle {
                        width: 5,
                        border_radius: Default::default(),
                    },
                    color: ACCENT_DARK,
                    border_width: 0.0,
                    border_color: PRIMARY_DARK,
                },
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> slider::Appearance {
        self.active(style)
    }

    fn dragging(&self, style: &Self::Style) -> slider::Appearance {
        self.active(style)
    }
}
