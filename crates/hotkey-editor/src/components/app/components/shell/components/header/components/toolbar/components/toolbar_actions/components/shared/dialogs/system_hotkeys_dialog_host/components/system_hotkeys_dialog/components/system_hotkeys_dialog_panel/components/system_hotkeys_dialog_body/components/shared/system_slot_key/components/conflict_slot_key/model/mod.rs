use super::view::ConflictSlotKeyView;
use dioxus::prelude::*;

/// The conflict key glyph's props: just the bound key's label. Built by the
/// `SystemSlotKey` dispatcher from `SystemSlotKeyModel` when the slot is in a
/// binding conflict.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictSlotKeyModel {
    #[props(into)]
    pub label: String,
}

impl From<&ConflictSlotKeyView> for ConflictSlotKeyModel {
    fn from(view: &ConflictSlotKeyView) -> Self {
        let ConflictSlotKeyView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Model for ConflictSlotKeyModel {
    type View = ConflictSlotKeyView;
}
