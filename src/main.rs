mod gui;

use gui::{widgets::ZeroPassTheme, UI};
use iced::{clipboard, executor, window, Application, Command, Settings};
use zero_pass_backend::{encrypt::PasswordBuilder, Methods};

const WINDOW_WIDTH: f32 = 500.0;
const WINDOW_HEIGHT: f32 = 600.0;
const TITLE: &str = "Zero Pass";

fn main() -> iced::Result {
    ZeroPass::run(Settings {
        fonts: vec![include_bytes!("../fonts/zero-pass-icons.ttf")
            .as_slice()
            .into()],
        window: iced::window::Settings {
            size: iced::Size {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
            },
            resizable: false,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    })
}

struct ZeroPass {
    unique: String,
    variable: String,
    methods: Vec<(u8, Methods)>,
    result: String,
    theme: ZeroPassTheme,
}
#[derive(Debug, Clone)]
pub enum Message {
    UniqueChange(String),
    VariableChange(String),
    Generate,
    PasswordIsGenerated(String),
    MethodSelect(usize, String),
    RepeatChange(usize, u8),
    AddMethod,
    RemoveMethod,
    CopyResult(String),
    ToggleTheme,
    CloseWindow,
}

impl Application for ZeroPass {
    type Message = Message;
    type Flags = ();
    type Theme = ZeroPassTheme;
    type Executor = executor::Default;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                unique: Default::default(),
                variable: Default::default(),
                methods: vec![(1, Methods::Base64)],
                result: Default::default(),
                theme: Self::Theme::default(),
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        TITLE.into()
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
            Message::Generate => Command::perform(
                generate_password(
                    self.unique.clone(),
                    self.variable.clone(),
                    self.methods.clone(),
                ),
                Message::PasswordIsGenerated,
            ),
            Message::PasswordIsGenerated(result) => {
                self.result = result;

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
            Message::CopyResult(result) => {
                // TODO: copy generated text
                clipboard::write(result)
            }
            Message::ToggleTheme => {
                self.theme = match self.theme {
                    Self::Theme::Dark => Self::Theme::Light,
                    Self::Theme::Light => Self::Theme::Dark
                };

                Command::none()
            }
            Message::CloseWindow => {
                window::close(window::Id::MAIN)
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, ZeroPassTheme> {
        UI::build(self)
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}

async fn generate_password(
    unique: String,
    variable: String,
    methods: Vec<(u8, Methods)>,
) -> String {
    let mut result = PasswordBuilder::new()
        .unique(unique.clone())
        .variable(variable.clone());
    for method in methods {
        result = result
            .repeat(method.0)
            .method_ptr(method.1.to_method())
            .await
            .unwrap();
    }

    result.build()
}
