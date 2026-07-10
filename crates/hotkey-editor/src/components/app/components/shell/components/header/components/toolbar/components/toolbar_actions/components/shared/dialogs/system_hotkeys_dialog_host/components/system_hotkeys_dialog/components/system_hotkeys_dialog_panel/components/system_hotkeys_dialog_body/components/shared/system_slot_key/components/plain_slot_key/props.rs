use super::view::PlainSlotKeyView;
use dioxus::prelude::*;

/// The plain (non-conflict) key glyph's props: just the bound key's label. Built by
/// the `SystemSlotKey` dispatcher from `SystemSlotKeyProps`.
#[derive(Props, Clone, PartialEq)]
pub struct PlainSlotKeyProps {
    #[props(into)]
    pub label: String,
}

impl From<&PlainSlotKeyView> for PlainSlotKeyProps {
    fn from(view: &PlainSlotKeyView) -> Self {
        let PlainSlotKeyView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Props for PlainSlotKeyProps {
    type View = PlainSlotKeyView;
}
