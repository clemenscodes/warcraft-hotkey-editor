use dioxus::prelude::*;
use warcraft_keybinds::Mana;

/// The mana value leaf's input: the unit's resolved mana pool.
#[derive(Props, Clone, PartialEq)]
pub struct ManaValueProps {
    pub value: Mana,
}
