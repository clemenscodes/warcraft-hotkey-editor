use super::view::ManaRegenRowView;
use dioxus::prelude::*;
use warcraft_keybinds::ManaRegen;

/// The mana regeneration row's input: the unit's resolved mana regeneration rate.
#[derive(Props, Clone, PartialEq)]
pub struct ManaRegenRowProps {
    pub value: ManaRegen,
}

impl From<&ManaRegenRowView> for ManaRegenRowProps {
    fn from(view: &ManaRegenRowView) -> Self {
        let ManaRegenRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Props for ManaRegenRowProps {
    type View = ManaRegenRowView;
}
