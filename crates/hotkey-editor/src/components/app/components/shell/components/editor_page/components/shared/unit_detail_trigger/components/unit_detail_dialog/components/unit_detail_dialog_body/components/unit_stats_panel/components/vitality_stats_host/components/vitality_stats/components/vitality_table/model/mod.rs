use super::view::VitalityTableView;
use dioxus::prelude::*;
use warcraft_api::{HitPoints, HitPointsRegen, Mana, ManaRegen};

#[derive(Props, Clone, PartialEq)]
pub struct VitalityTableModel {
    pub hit_points: HitPoints,
    pub hit_points_regen: HitPointsRegen,
    pub mana: Mana,
    pub mana_regen: ManaRegen,
}

impl From<&VitalityTableView> for VitalityTableModel {
    fn from(view: &VitalityTableView) -> Self {
        let VitalityTableView {
            hit_points,
            hit_points_regen,
            mana,
            mana_regen,
        } = view.clone();
        Self {
            hit_points,
            hit_points_regen,
            mana,
            mana_regen,
        }
    }
}

impl ddd::Layered for VitalityTableModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for VitalityTableModel {
    type View = VitalityTableView;
}
