use super::view::CombatRowsView;
use dioxus::prelude::*;
use warcraft_api::AttackType;
use warcraft_api::{AttackRange, AttackSpeed, DamagePerSecond, DamageRange};

/// The combat rows' input: the five attack figures the rows present (the icon stays
/// with the column). `damage_per_second` is `None` when the attack has no real
/// cooldown; `attack_type` feeds both the attack-type row and the matchup grid.
#[derive(Props, Clone, PartialEq)]
pub struct CombatRowsModel {
    pub damage: DamageRange,
    pub range: AttackRange,
    pub speed: AttackSpeed,
    pub damage_per_second: Option<DamagePerSecond>,
    pub attack_type: AttackType,
}

impl From<&CombatRowsView> for CombatRowsModel {
    fn from(view: &CombatRowsView) -> Self {
        let CombatRowsView {
            damage,
            range,
            speed,
            damage_per_second,
            attack_type,
        } = view.clone();
        Self {
            damage,
            range,
            speed,
            damage_per_second,
            attack_type,
        }
    }
}

impl ddd::Model for CombatRowsModel {
    type View = CombatRowsView;
}
