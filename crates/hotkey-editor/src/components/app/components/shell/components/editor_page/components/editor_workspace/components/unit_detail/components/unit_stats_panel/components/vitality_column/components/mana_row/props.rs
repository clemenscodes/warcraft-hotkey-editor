use dioxus::prelude::*;
use warcraft_keybinds::Mana;

/// The mana row's input: the unit's resolved mana pool at the selected level.
#[derive(Props, Clone, PartialEq)]
pub struct ManaRowProps {
    pub value: Mana,
}
