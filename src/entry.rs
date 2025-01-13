use iced::widget::{button, column, text};
use iced::Element;
use time::Date;

pub struct Entry {
    action: String,
    date_added: Date
}

#[derive(Debug, Clone)]
pub enum EntryAction {
    Add,
    Delete
}

impl Entry {
    pub fn view(entries: Vec<Entry>) -> Element<EntryAction> {
        column![
            text("This is an entry").size(20),
            button("add entry").on_press(EntryAction::Add),
        ]
        .spacing(10)
        .into()
    }

    pub fn update(&mut self, message: EntryAction) {
        match message {
            EntryAction::Add => {
                panic!()
            }
            _ => panic!()
        }
    }
}
