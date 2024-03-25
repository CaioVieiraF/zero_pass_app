mod gui;

use gui::{widgets::ZeroPassTheme, UI};
use iced::{executor, Application, Command, Settings};
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
pub enum Message {
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
    type Theme = ZeroPassTheme;
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
                self.unique = content.trim().into();

                Command::none()
            }
            Message::VariableChange(content) => {
                self.variable = content.trim().into();
                Command::none()
            }
            Message::Generate => {
                let mut result = PasswordBuilder::new()
                    .unique(self.unique.clone())
                    .variable(self.variable.clone());
                for method in &self.methods {
                    result = result
                        .repeat(method.0)
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

    fn view(&self) -> iced::Element<'_, Self::Message, ZeroPassTheme> {
        UI::build(self)
    }

    fn theme(&self) -> Self::Theme {
        ZeroPassTheme::Dark
    }
}
