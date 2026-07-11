use super::view::CombatColumnView;
use dioxus::prelude::*;
use warcraft_keybinds::AttackStatistics;

/// The combat column's input: the unit's attack profile, or `None` when it cannot
/// attack (the column then renders nothing).
#[derive(Props, Clone, PartialEq)]
pub struct CombatColumnModel {
    pub attack: Option<AttackStatistics>,
}

impl From<&CombatColumnView> for CombatColumnModel {
    fn from(view: &CombatColumnView) -> Self {
        let CombatColumnView { attack } = view.clone();
        Self { attack }
    }
}

impl ddd::Model for CombatColumnModel {
    type View = CombatColumnView;
}
