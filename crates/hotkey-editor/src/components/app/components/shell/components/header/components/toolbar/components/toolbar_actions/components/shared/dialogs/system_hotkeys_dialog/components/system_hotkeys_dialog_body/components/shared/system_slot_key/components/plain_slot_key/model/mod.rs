use super::view::PlainSlotKeyView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PlainSlotKeyModel {
    #[props(into)]
    pub label: String,
}

impl From<&PlainSlotKeyView> for PlainSlotKeyModel {
    fn from(view: &PlainSlotKeyView) -> Self {
        let PlainSlotKeyView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Model for PlainSlotKeyModel {
    type View = PlainSlotKeyView;
}
