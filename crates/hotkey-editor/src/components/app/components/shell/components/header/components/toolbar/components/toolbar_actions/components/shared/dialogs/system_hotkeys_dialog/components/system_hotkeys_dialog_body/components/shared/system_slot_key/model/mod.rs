use super::view::SystemSlotKeyView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SystemSlotKeyModel {
    #[props(into)]
    pub label: String,
    pub conflict: bool,
}

impl From<&SystemSlotKeyView> for SystemSlotKeyModel {
    fn from(view: &SystemSlotKeyView) -> Self {
        let SystemSlotKeyView { label, conflict } = view.clone();
        Self { label, conflict }
    }
}

impl ddd::Model for SystemSlotKeyModel {
    type View = SystemSlotKeyView;
}
