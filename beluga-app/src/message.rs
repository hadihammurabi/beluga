use iced::Theme;

#[derive(Debug, Clone)]
pub enum Message {
    // Next,
    // Previous,
    // ExplainToggled(bool),
    ThemeSelected(Theme),
}
