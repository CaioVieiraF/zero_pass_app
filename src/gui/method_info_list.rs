use iced::{
    alignment::Horizontal,
    theme,
    widget::{column, container, pick_list, row, scrollable, slider, text, Column},
    Length,
};
use zero_pass_backend::Methods;

use crate::Message;

use super::widgets::Button;

pub fn method_info_list<'a>(methods: &[(u8, Methods)]) -> iced::Element<'a, Message> {
    let add_method = Button::new(
        text("+")
            .horizontal_alignment(Horizontal::Center)
            .width(Length::Fill),
    )
    .on_press(Message::AddMethod)
    .width(Length::Fill)
    .style(theme::Button::Positive);

    let remove_method = Button::new(
        text("-")
            .horizontal_alignment(Horizontal::Center)
            .width(Length::Fill),
    )
    .on_press(Message::RemoveMethod)
    .width(Length::Fill)
    .style(theme::Button::Destructive);

    let mut methods_picker = vec![];

    for (i, method) in methods.iter().enumerate() {
        let index = i;
        let repeat_slider = slider(1..=128, methods[i].0, move |r| {
            Message::RepeatChange(index, r)
        });
        methods_picker.push(
            row![
                pick_list(
                    Methods::get_methods()
                        .iter()
                        .map(|m| String::from(*m))
                        .collect::<Vec<_>>(),
                    Some(format!("{:?}", method.1)),
                    move |m| Message::MethodSelect(i, m)
                ),
                column![
                    repeat_slider,
                    container(text(format!("{}", methods[i].0)))
                        .width(Length::Fill)
                        .center_x()
                ]
            ]
            .spacing(20)
            .into(),
        );
    }

    container(scrollable(column![
        Column::with_children(methods_picker).spacing(5).padding(5),
        row![add_method, remove_method]
    ]))
    .width(Length::Fill)
    .style(iced::theme::Container::Box)
    .into()
}

