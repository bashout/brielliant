use std::path::Path;
use iced::widget::{button, column, text};
use iced::Element;
use chrono::Local;

#[derive(Default)]
pub struct Line {
    action: String,
    date_added: Local
}

#[derive(Debug, Clone)]
pub enum LineAction {
    Add,
    Delete
}

impl Line {
    pub fn view(sounds: Vec<Line>) -> Element<LineAction> {
        column![
            text("This is a sound").size(20),
            button("Play Sound").on_press(LineAction::Add),
        ]
        .spacing(10)
        .into()
    }

    pub fn update(&mut self, message: LineAction) {
        match message {
            LineAction::Add => {
                panic!()
            }
            _ => panic!()
        }
    }
}
