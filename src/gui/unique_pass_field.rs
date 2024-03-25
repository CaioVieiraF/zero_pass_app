use super::widgets::{text::TextStyle, Text, TextInput, ZeroPassTheme};
use crate::Message;
use iced::widget::column;

pub fn unique_pass_field<'a>(unique: &str) -> iced::Element<'a, Message, ZeroPassTheme> {
    let unique_field = TextInput::new("Unique", unique).on_input(Message::UniqueChange);

    column![
        Text::new("Unique pass").style(TextStyle::Dark),
        unique_field
    ]
    .into()
}
