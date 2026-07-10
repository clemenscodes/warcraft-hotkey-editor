use dioxus::prelude::*;

/// The conflict key glyph's props: just the bound key's label. Built by the
/// `SystemSlotKey` dispatcher from `SystemSlotKeyProps` when the slot is in a
/// binding conflict.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictSlotKeyProps {
    #[props(into)]
    pub label: String,
}
