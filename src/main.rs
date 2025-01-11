mod line;

pub fn main() -> iced::Result {
    iced::run(
        "Brielliant Planner",
        line::Line::update,
        line::Line::view,
    )
}
