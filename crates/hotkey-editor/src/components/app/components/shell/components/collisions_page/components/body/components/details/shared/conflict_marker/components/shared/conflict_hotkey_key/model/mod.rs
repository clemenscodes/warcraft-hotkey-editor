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

impl ddd::Layered for ConflictHotkeyKeyModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for ConflictHotkeyKeyModel {
    type View = ConflictHotkeyKeyView;
}
