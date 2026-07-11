use super::view::ConflictHotkeyKeyView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictHotkeyKeyModel {
    #[props(into)]
    pub text: String,
}

impl From<&ConflictHotkeyKeyView> for ConflictHotkeyKeyModel {
    fn from(view: &ConflictHotkeyKeyView) -> Self {
        let ConflictHotkeyKeyView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for ConflictHotkeyKeyModel {
    type View = ConflictHotkeyKeyView;
}
