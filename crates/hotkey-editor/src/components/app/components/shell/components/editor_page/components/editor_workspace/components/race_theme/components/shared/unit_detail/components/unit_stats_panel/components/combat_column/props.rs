use super::view::CombatColumnView;
use dioxus::prelude::*;
use warcraft_keybinds::AttackStatistics;

/// The combat column's input: the unit's attack profile, or `None` when it cannot
/// attack (the column then renders nothing).
#[derive(Props, Clone, PartialEq)]
pub struct CombatColumnProps {
    pub attack: Option<AttackStatistics>,
}

impl From<&CombatColumnView> for CombatColumnProps {
    fn from(view: &CombatColumnView) -> Self {
        let CombatColumnView { attack } = view.clone();
        Self { attack }
    }
}

impl ddd::Props for CombatColumnProps {
    type View = CombatColumnView;
}
