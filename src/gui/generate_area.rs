use iced::{
    alignment::Horizontal,
    widget::{column, container, row},
    Length,
};

use crate::Message;

use super::widgets::{
    button::ButtonStyle, copy_icon, text::TextStyle, text_input::TextInputStyle, Button, Text,
    TextInput, ZeroPassTheme,
};

pub fn generate_area<'a>(result: &String) -> iced::Element<'a, Message, ZeroPassTheme> {
    let gen_button = container(
        Button::new(
            Text::new("Generate password")
                .horizontal_alignment(Horizontal::Center)
                .style(TextStyle::Light)
                .width(Length::Fill),
        )
        .on_press(Message::Generate)
        .padding(10)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .center_x();

    let result = row!(
        TextInput::new("The result will appear here", result)
            .padding(10)
            .style(TextInputStyle::Copy)
            .size(20),
        Button::new(copy_icon())
            .on_press(Message::CopyResult(result.to_string()))
            .style(ButtonStyle::Copy)
            .padding(12)
    );

    container(column![gen_button, result].spacing(10)).into()
}
