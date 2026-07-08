use dioxus::prelude::*;
use warcraft_keybinds::AttackSpeed;

/// The attack speed row's input: the unit's attack cooldown.
#[derive(Props, Clone, PartialEq)]
pub struct AttackSpeedRowProps {
    pub value: AttackSpeed,
}
