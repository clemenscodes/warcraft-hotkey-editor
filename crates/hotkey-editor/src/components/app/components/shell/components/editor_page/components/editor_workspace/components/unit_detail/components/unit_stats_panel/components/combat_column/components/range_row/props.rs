use dioxus::prelude::*;
use warcraft_keybinds::AttackRange;

/// The range row's input: the attack's reach.
#[derive(Props, Clone, PartialEq)]
pub struct RangeRowProps {
    pub range: AttackRange,
}
