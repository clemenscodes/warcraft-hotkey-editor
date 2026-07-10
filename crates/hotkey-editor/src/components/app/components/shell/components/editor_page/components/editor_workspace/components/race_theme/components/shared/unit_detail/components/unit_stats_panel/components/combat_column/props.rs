use dioxus::prelude::*;
use warcraft_keybinds::AttackStatistics;

/// The combat column's input: the unit's attack profile, or `None` when it cannot
/// attack (the column then renders nothing).
#[derive(Props, Clone, PartialEq)]
pub struct CombatColumnProps {
    pub attack: Option<AttackStatistics>,
}
