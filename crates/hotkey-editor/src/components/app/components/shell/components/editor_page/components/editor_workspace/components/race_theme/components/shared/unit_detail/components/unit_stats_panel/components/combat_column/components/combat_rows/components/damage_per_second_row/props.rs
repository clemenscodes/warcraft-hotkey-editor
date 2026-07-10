use dioxus::prelude::*;
use warcraft_keybinds::DamagePerSecond;

/// The damage-per-second row's input: the derived rate, or `None` when the attack has
/// no real cooldown (so a rate is undefined and the row is absent).
#[derive(Props, Clone, PartialEq)]
pub struct DamagePerSecondRowProps {
    pub value: Option<DamagePerSecond>,
}
