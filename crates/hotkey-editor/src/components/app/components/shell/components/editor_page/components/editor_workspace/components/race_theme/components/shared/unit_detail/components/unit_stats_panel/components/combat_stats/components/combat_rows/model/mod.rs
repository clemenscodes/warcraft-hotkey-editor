use super::view::CombatRowsView;
use dioxus::prelude::*;
use warcraft_api::AttackType;
use warcraft_api::{AttackRange, AttackSpeed, DamagePerSecond, DamageRange};

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
