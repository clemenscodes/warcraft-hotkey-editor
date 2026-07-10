use super::view::ConflictHotkeyKeyView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictHotkeyKeyProps {
    #[props(into)]
    pub text: String,
}

impl From<&ConflictHotkeyKeyView> for ConflictHotkeyKeyProps {
    fn from(view: &ConflictHotkeyKeyView) -> Self {
        let ConflictHotkeyKeyView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for ConflictHotkeyKeyProps {
    type View = ConflictHotkeyKeyView;
}
