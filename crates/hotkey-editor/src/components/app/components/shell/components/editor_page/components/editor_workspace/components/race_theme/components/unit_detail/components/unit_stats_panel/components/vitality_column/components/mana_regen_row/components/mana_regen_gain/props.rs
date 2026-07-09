use dioxus::prelude::*;
use warcraft_keybinds::ManaRegen;

/// The mana-regeneration gain leaf's input: the unit's resolved mana regeneration.
#[derive(Props, Clone, PartialEq)]
pub struct ManaRegenGainProps {
    pub value: ManaRegen,
}
