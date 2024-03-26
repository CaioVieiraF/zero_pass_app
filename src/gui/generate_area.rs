use iced::{alignment::Horizontal, widget::row, Length};

use crate::Message;

use super::widgets::{
    button::ButtonStyle, text_input::TextInputStyle, Button, Column, Container, Text, TextInput,
    ZeroPassTheme, copy_icon
};

pub fn generate_area<'a>(result: &String) -> iced::Element<'a, Message, ZeroPassTheme> {
    let gen_button = Container::new(
        Button::new(
            Text::new("Generate password")
                .horizontal_alignment(Horizontal::Center)
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
            .style(TextInputStyle::Copy)
            .padding(10)
            .size(20),
        Button::new(copy_icon())
            .on_press(Message::CopyResult(result.to_string()))
            .style(ButtonStyle::Copy)
            .padding(12)
    );

    Container::new(Column::with_children([gen_button.into(), result.into()]).spacing(10)).into()
}
