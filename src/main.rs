mod counter;

pub fn main() -> iced::Result {
    iced::run(
        "A cool counter",
        counter::Counter::update,
        counter::Counter::view,
    )
}
