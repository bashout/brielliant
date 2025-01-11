mod line;

pub fn main() -> iced::Result {
    iced::run(
        "Brielliant Planner",
        line::Entry::update,
        line::Entry::view,
    )
}
