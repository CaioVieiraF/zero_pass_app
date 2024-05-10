use iced::border::Radius;
use iced::widget::button::{self, Appearance};
use iced::Background;

use super::{ZeroPassTheme, ACCENT_DARK, PRIMARY_DARK, SECONDARY_DARK};

// Button variants used on the app
#[derive(Default, Debug, Clone)]
pub enum ButtonStyle {
    #[default]
    Default,
    AddMethod,
    RemoveMethod,
    Copy,
}

// Implementing button stylesheet for the custom app theme
impl button::StyleSheet for ZeroPassTheme {
    type Style = ButtonStyle;

    // Handle the style based on the button type on normal state
    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            ButtonStyle::Default => generate_button_active_style(),
            ButtonStyle::AddMethod => add_method_button_active_style(),
            ButtonStyle::RemoveMethod => remove_method_button_active_style(),
            ButtonStyle::Copy => copy_button_active_style(),
        }
    }

    // Handle the style based on the button type when hovered
    fn hovered(&self, style: &Self::Style) -> Appearance {
        match style {
            ButtonStyle::Default => generate_button_hovered_style(),
            ButtonStyle::AddMethod => add_method_button_hovered_style(),
            ButtonStyle::RemoveMethod => remove_method_button_hovered_style(),
            ButtonStyle::Copy => copy_button_active_style(),
        }
    }

    // Handle the style based on the button type while it is pressed
    fn pressed(&self, style: &Self::Style) -> Appearance {
        match style {
            ButtonStyle::Default => generate_button_active_style(),
            ButtonStyle::AddMethod => add_method_button_pressed_style(),
            ButtonStyle::RemoveMethod => remove_method_button_pressed_style(),
            ButtonStyle::Copy => copy_button_active_style(),
        }
    }
}

// ACTIVE STYLES

fn generate_button_active_style() -> Appearance {
    Appearance {
        background: Some(Background::Color(PRIMARY_DARK)),
        border: iced::Border {
            color: PRIMARY_DARK,
            radius: Radius::from(4),
            ..Default::default()
        },
        ..Appearance::default()
    }
}

fn add_method_button_active_style() -> Appearance {
    Appearance {
        background: Some(Background::Color(SECONDARY_DARK)),
        border: iced::Border {
            color: SECONDARY_DARK,
            radius: Radius::from([4.0, 0.0, 0.0, 4.0]),
            ..Default::default()
        },
        ..Appearance::default()
    }
}

fn remove_method_button_active_style() -> Appearance {
    Appearance {
        background: Some(Background::Color(SECONDARY_DARK)),
        border: iced::Border {
            color: SECONDARY_DARK,
            radius: Radius::from([0.0, 4.0, 4.0, 0.0]),
            ..Default::default()
        },
        ..Appearance::default()
    }
}

fn copy_button_active_style() -> Appearance {
    Appearance {
        background: Some(Background::Color(ACCENT_DARK)),
        border: iced::Border {
            radius: Radius::from([0.0, 4.0, 4.0, 0.0]),
            width: 1.0,
            color: ACCENT_DARK,
        },
        ..Appearance::default()
    }
}

// HOVERED STYLES

fn generate_button_hovered_style() -> Appearance {
    Appearance {
        background: Some(Background::Color(PRIMARY_DARK)),
        border: iced::Border {
            color: PRIMARY_DARK,
            width: 3.0,
            radius: Radius::from(4),
        },
        shadow: iced::Shadow {
            color: SECONDARY_DARK,
            blur_radius: 8.0,
            offset: iced::Vector { x: 0.0, y: 0.0 },
        },
        ..Appearance::default()
    }
}

fn add_method_button_hovered_style() -> Appearance {
    Appearance {
        background: Some(Background::Color(SECONDARY_DARK)),
        border: iced::Border {
            color: ACCENT_DARK,
            width: 2.0,
            radius: Radius::from([4.0, 0.0, 0.0, 4.0]),
        },
        ..Appearance::default()
    }
}

fn remove_method_button_hovered_style() -> Appearance {
    Appearance {
        background: Some(Background::Color(SECONDARY_DARK)),
        border: iced::Border {
            color: ACCENT_DARK,
            width: 2.0,
            radius: Radius::from([0.0, 4.0, 4.0, 0.0]),
        },
        ..Appearance::default()
    }
}

// PRESSED STYLES

fn add_method_button_pressed_style() -> Appearance {
    Appearance {
        background: Some(Background::Color(ACCENT_DARK)),
        border: iced::Border {
            color: SECONDARY_DARK,
            radius: Radius::from([4.0, 0.0, 0.0, 4.0]),
            ..Default::default()
        },
        ..Appearance::default()
    }
}

fn remove_method_button_pressed_style() -> Appearance {
    Appearance {
        background: Some(Background::Color(ACCENT_DARK)),
        border: iced::Border {
            color: SECONDARY_DARK,
            radius: Radius::from([0.0, 4.0, 4.0, 0.0]),
            ..Default::default()
        },
        ..Appearance::default()
    }
}
