mod entry;

pub fn main() -> iced::Result {
    iced::run(
        "Brielliant Planner",
        entry::Entry::update,
        entry::Entry::view,
    )
}
