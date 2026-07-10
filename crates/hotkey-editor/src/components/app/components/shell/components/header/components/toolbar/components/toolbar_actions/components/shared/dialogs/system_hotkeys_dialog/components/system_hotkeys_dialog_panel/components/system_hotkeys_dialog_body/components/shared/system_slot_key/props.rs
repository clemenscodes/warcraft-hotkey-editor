use dioxus::prelude::*;

/// A slot's key glyph: the bound key's label, whether the slot is a compact
/// (control-group) cell, and whether the slot is in a binding conflict (which
/// turns the key red).
#[derive(Props, Clone, PartialEq)]
pub struct SystemSlotKeyProps {
    #[props(into)]
    pub label: String,
    pub compact: bool,
    pub conflict: bool,
}
