use iced::widget::{button, column, text};
use iced::Element;

#[derive(Default)]
pub struct Counter {
    value: i64,
}

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
}

impl Counter {
    pub fn view(counter: &Counter) -> Element<Message> {
        column![
            text(counter.value).size(20),
            button("Increment").on_press(Message::Increment),
        ]
        .spacing(10)
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
        }
    }
}
