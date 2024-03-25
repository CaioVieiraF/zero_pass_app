use iced::widget::container::{self, Appearance};

use super::ZeroPassTheme;

#[derive(Default)]
pub enum ContainerStyle {
    #[default]
    Default,
}

impl container::StyleSheet for ZeroPassTheme {
    type Style = ContainerStyle;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match style {
            ContainerStyle::Default => Appearance::default(),
        }
    }
}
