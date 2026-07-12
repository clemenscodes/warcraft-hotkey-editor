use super::view::DamageMatchupRowView;
use dioxus::prelude::*;
use warcraft_api::AttackType;

/// The attacker's damage matchup grid: how its attack fares against each defense.
#[derive(Props, Clone, PartialEq)]
pub struct DamageMatchupRowModel {
    pub attack_type: AttackType,
}

impl From<&DamageMatchupRowView> for DamageMatchupRowModel {
    fn from(view: &DamageMatchupRowView) -> Self {
        let DamageMatchupRowView { attack_type } = view.clone();
        Self { attack_type }
    }
}

impl ddd::Model for DamageMatchupRowModel {
    type View = DamageMatchupRowView;
}
