use dioxus::prelude::*;
use warcraft_keybinds::ManaRegen;

/// The mana regeneration row's input: the unit's resolved mana regeneration rate.
#[derive(Props, Clone, PartialEq)]
pub struct ManaRegenRowProps {
    pub value: ManaRegen,
}
