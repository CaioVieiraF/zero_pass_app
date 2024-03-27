use iced::{alignment::Horizontal, Length};
use zero_pass_backend::Methods;

use crate::Message;

use super::widgets::{
    button::ButtonStyle, container::ContainerStyle, text::TextStyle, Button, Column, Container,
    PickList, Row, Scrollable, Slider, Text, ZeroPassTheme,
};

pub fn method_info_list<'a>(
    methods: &[(u8, Methods)],
) -> iced::Element<'a, Message, ZeroPassTheme> {
    let add_method = Button::new(
        Text::new("+")
            .horizontal_alignment(Horizontal::Center)
            .width(Length::Fill)
            .style(TextStyle::Dark),
    )
    .on_press(Message::AddMethod)
    .width(Length::Fill)
    .style(ButtonStyle::AddMethod);

    let remove_method = Button::new(
        Text::new("-")
            .horizontal_alignment(Horizontal::Center)
            .width(Length::Fill)
            .style(TextStyle::Dark),
    )
    .on_press(Message::RemoveMethod)
    .width(Length::Fill)
    .style(ButtonStyle::RemoveMethod);

    let mut methods_picker = vec![];

    for (i, method) in methods.iter().enumerate() {
        let index = i;
        let repeat_slider = Slider::new(1..=128, methods[i].0, move |r| {
            Message::RepeatChange(index, r)
        });
        methods_picker.push(
            Row::with_children([
                Column::with_children([PickList::new(
                    Methods::get_methods()
                        .iter()
                        .map(|m| String::from(*m))
                        .collect::<Vec<_>>(),
                    Some(format!("{:?}", method.1)),
                    move |m| Message::MethodSelect(i, m),
                )
                .into()])
                .into(),
                Column::with_children([
                    Container::new(repeat_slider).into(),
                    Container::new(Text::new(format!("{}", methods[i].0)).style(TextStyle::Dark))
                        .width(Length::Fill)
                        .center_x()
                        .into(),
                ])
                .into(),
            ])
            .spacing(20)
            .into(),
        );
    }

    Container::new(Scrollable::new(Column::with_children([
        Column::with_children(methods_picker)
            .spacing(5)
            .padding(5)
            .into(),
        Row::with_children([add_method.into(), remove_method.into()]).into(),
    ])))
    .width(Length::Fill)
    .style(ContainerStyle::default())
    .into()
}
