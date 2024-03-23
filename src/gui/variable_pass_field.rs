use iced::widget::{column, text};

use crate::Message;

use super::widgets::TextInput;

pub fn variable_pass_field<'a>(variable: &str) -> iced::Element<'a, Message> {
    let variable_field = TextInput::new("Variable", variable).on_input(Message::VariableChange);

    column![text("Variable pass"), variable_field].into()
}

