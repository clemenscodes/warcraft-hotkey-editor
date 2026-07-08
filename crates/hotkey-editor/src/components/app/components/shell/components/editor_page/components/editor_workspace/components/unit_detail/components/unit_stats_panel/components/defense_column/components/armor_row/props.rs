use dioxus::prelude::*;
use warcraft_keybinds::Armor;

/// The armor row's input: the unit's resolved armor.
#[derive(Props, Clone, PartialEq)]
pub struct ArmorRowProps {
    pub value: Armor,
}
