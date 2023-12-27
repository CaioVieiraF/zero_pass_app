use iced::{
    alignment::Horizontal,
    executor, theme,
    widget::{
        button, column, container, pick_list, row, scrollable, slider, text, text_input, Column,
    },
    Application, Command, Length, Settings, Theme,
};
use zero_pass_backend::{encrypt::PasswordBuilder, Methods};

fn main() -> iced::Result {
    ZeroPass::run(Settings::default())
}

struct ZeroPass {
    unique: String,
    variable: String,
    methods: Vec<(u8, Methods)>,
    result: String,
}
#[derive(Debug, Clone)]
enum Message {
    UniqueChange(String),
    VariableChange(String),
    Generate,
    MethodSelect(usize, String),
    RepeatChange(usize, u8),
    AddMethod,
    RemoveMethod,
}

impl Application for ZeroPass {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = executor::Default;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                unique: String::default(),
                variable: String::default(),
                methods: vec![(1, Methods::Base64)],
                result: String::default(),
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("Zero Pass")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::UniqueChange(content) => {
                self.unique = content;

                Command::none()
            }
            Message::VariableChange(content) => {
                self.variable = content;
                Command::none()
            }
            Message::Generate => {
                let mut result = PasswordBuilder::new()
                    .unique(self.unique.clone())
                    .variable(self.variable.clone());
                for method in &self.methods {
                    result = result
                        .repeat(method.0.clone())
                        .method_ptr(method.1.to_method())
                        .unwrap();
                }

                self.result = result.build();
                Command::none()
            }
            Message::MethodSelect(i, method) => {
                self.methods[i].1 = Methods::try_from(method).unwrap();

                Command::none()
            }
            Message::RepeatChange(i, value) => {
                self.methods[i].0 = value;

                Command::none()
            }
            Message::AddMethod => {
                self.methods.push((1, Methods::Base64));

                Command::none()
            }
            Message::RemoveMethod => {
                if self.methods.len() > 1 {
                    self.methods.pop();
                }

                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let unique_field = text_input("Unique", &self.unique).on_input(Message::UniqueChange);
        let variable_field =
            text_input("Variable", &self.variable).on_input(Message::VariableChange);
        let gen_button = container(
            button(
                text("Generate password")
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill),
            )
            .on_press(Message::Generate)
            .width(Length::Fill),
        )
        .width(Length::Fill)
        .center_x();
        let result = text(&self.result);
        let add_method = button(
            text("+")
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill),
        )
        .on_press(Message::AddMethod)
        .width(Length::Fill)
        .style(theme::Button::Positive);
        let remove_method = button(
            text("-")
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill),
        )
        .on_press(Message::RemoveMethod)
        .width(Length::Fill)
        .style(theme::Button::Destructive);
        let mut methods_picker = vec![];

        for (i, method) in self.methods.iter().enumerate().into_iter() {
            let index = i.clone();
            let repeat_slider = slider(1..=128, self.methods[i].0, move |r| {
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
                        container(text(format!("{}", self.methods[i].0)))
                            .width(Length::Fill)
                            .center_x()
                    ]
                ]
                .spacing(20)
                .into(),
            );
        }
        container(
            column![
                column![text("Unique pass"), unique_field],
                column![text("Variable pass"), variable_field],
                container(scrollable(column![
                    Column::with_children(methods_picker).spacing(5).padding(5),
                    row![add_method, remove_method]
                ]))
                .width(Length::Fill)
                .style(iced::theme::Container::Box),
                container(column![gen_button, result])
            ]
            .spacing(20),
        )
        .padding(20)
        .height(Length::Fill)
        .center_y()
        .into()
    }

    fn theme(&self) -> iced::Theme {
        Theme::Dark
    }
}
