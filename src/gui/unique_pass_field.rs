use super::widgets::TextInput;
use crate::Message;
use iced::widget::{column, text};

pub fn unique_pass_field<'a>(unique: &str) -> iced::Element<'a, Message> {
    let unique_field = TextInput::new("Unique", unique).on_input(Message::UniqueChange);

    column![text("Unique pass"), unique_field].into()
}

