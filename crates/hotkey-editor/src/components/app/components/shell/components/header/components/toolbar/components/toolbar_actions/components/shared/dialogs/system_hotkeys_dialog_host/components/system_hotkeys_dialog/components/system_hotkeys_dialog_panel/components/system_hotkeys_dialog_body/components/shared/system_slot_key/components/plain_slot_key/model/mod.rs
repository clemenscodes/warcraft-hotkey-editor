use super::view::PlainSlotKeyView;
use dioxus::prelude::*;

/// The plain (non-conflict) key glyph's props: just the bound key's label. Built by
/// the `SystemSlotKey` dispatcher from `SystemSlotKeyModel`.
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
