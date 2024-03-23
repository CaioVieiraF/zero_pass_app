use iced::{
    alignment::Horizontal,
    widget::{column, container, text},
    Length,
};

use crate::Message;

use super::widgets::{Button, TextInput};

pub fn generate_area<'a>(result: &String) -> iced::Element<'a, Message> {
    let gen_button = container(
        Button::new(
            text("Generate password")
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill),
        )
        .on_press(Message::Generate)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .center_x();

    let result = TextInput::new("The result will appear here", result);

    container(column![gen_button, result].spacing(10)).into()
}

