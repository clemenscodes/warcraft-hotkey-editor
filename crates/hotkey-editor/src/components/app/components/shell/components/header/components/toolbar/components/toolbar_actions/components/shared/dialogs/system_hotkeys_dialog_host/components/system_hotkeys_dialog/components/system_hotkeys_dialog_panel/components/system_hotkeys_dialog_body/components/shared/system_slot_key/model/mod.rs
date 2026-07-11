use super::view::SystemSlotKeyView;
use dioxus::prelude::*;

/// A slot's key glyph: the bound key's label and whether the slot is in a binding
/// conflict (which recolours the glyph danger-red). The tighter control-group glyph
/// size is owned by the parent size container, so no density flag rides here.
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
