mod generate_area;
mod method_info_list;
mod unique_pass_field;
mod variable_pass_field;
pub mod widgets;

use crate::ZeroPass;

use self::widgets::ZeroPassTheme;

use super::Message;
use iced::{
    widget::{column, container},
    Length,
};

pub struct UI;

impl UI {
    pub fn build<'a>(zero_pass: &ZeroPass) -> iced::Element<'a, Message, ZeroPassTheme> {
        container(
            column![
                unique_pass_field::unique_pass_field(&zero_pass.unique),
                variable_pass_field::variable_pass_field(&zero_pass.variable),
                method_info_list::method_info_list(&zero_pass.methods),
                generate_area::generate_area(&zero_pass.result)
            ]
            .spacing(10),
        )
        .padding(20)
        .height(Length::Fill)
        .center_y()
        .into()
    }
}
