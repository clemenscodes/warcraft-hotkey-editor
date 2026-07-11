use warcraft_api::AttackType;
use warcraft_keybinds::{AttackRange, AttackSpeed, DamagePerSecond, DamageRange};

/// The published `View` contract mirroring [`CombatRowsModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CombatRowsView {
    pub damage: DamageRange,
    pub range: AttackRange,
    pub speed: AttackSpeed,
    pub damage_per_second: Option<DamagePerSecond>,
    pub attack_type: AttackType,
}

impl ddd::View for CombatRowsView {}
