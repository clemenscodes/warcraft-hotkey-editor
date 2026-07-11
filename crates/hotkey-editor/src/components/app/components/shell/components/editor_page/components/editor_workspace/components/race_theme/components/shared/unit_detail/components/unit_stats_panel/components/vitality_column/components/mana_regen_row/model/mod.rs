use super::view::ManaRegenRowView;
use dioxus::prelude::*;
use warcraft_keybinds::ManaRegen;

/// The mana regeneration row's input: the unit's resolved mana regeneration rate.
#[derive(Props, Clone, PartialEq)]
pub struct ManaRegenRowModel {
    pub value: ManaRegen,
}

impl From<&ManaRegenRowView> for ManaRegenRowModel {
    fn from(view: &ManaRegenRowView) -> Self {
        let ManaRegenRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Model for ManaRegenRowModel {
    type View = ManaRegenRowView;
}
