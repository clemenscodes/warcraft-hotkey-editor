use warcraft_keybinds::AttackStatistics;

/// The published `View` contract mirroring [`CombatColumnProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CombatColumnView {
    pub attack: Option<AttackStatistics>,
}

impl ddd::View for CombatColumnView {}
