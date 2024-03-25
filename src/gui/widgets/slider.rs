use iced::widget::slider;

use super::{ZeroPassTheme, ACCENT, BACKGROUND, PRIMARY, SECONDARY};

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
                    colors: (SECONDARY, BACKGROUND),
                    width: 1.0,
                    border_radius: Default::default(),
                },
                handle: slider::Handle {
                    shape: slider::HandleShape::Rectangle {
                        width: 5,
                        border_radius: Default::default(),
                    },
                    color: ACCENT,
                    border_width: 0.0,
                    border_color: PRIMARY,
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
