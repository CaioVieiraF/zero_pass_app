use iced::widget::column;

use crate::Message;

use super::widgets::{text::TextStyle, Text, TextInput, ZeroPassTheme};

pub fn variable_pass_field<'a>(variable: &str) -> iced::Element<'a, Message, ZeroPassTheme> {
    let variable_field = TextInput::new("Variable", variable)
        .on_input(Message::VariableChange)
        .size(20)
        .padding(10);

    column![
        Text::new("Variable pass").style(TextStyle::Dark),
        variable_field
    ]
    .into()
}
