use dioxus::prelude::*;
use warcraft_keybinds::DamageRange;

/// The damage row's input: the unit's attack damage range.
#[derive(Props, Clone, PartialEq)]
pub struct DamageRowProps {
    pub value: DamageRange,
}
