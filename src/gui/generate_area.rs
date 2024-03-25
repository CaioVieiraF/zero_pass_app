use iced::{alignment::Horizontal, Length};

use crate::Message;

use super::widgets::{Button, Column, Container, Text, TextInput, ZeroPassTheme};

pub fn generate_area<'a>(result: &String) -> iced::Element<'a, Message, ZeroPassTheme> {
    let gen_button = Container::new(
        Button::new(
            Text::new("Generate password")
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill),
        )
        .on_press(Message::Generate)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .center_x();

    let result = TextInput::new("The result will appear here", result);

    Container::new(Column::with_children([gen_button.into(), result.into()]).spacing(10)).into()
}
