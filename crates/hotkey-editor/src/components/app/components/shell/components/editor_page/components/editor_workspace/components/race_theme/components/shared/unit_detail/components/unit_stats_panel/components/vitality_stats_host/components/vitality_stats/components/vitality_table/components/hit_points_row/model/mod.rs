use super::view::HitPointsRowView;
use dioxus::prelude::*;
use warcraft_api::{HitPoints, HitPointsRegen};

#[derive(Props, Clone, PartialEq)]
pub struct HitPointsRowModel {
    pub value: HitPoints,
    pub regen: HitPointsRegen,
}

impl From<&HitPointsRowView> for HitPointsRowModel {
    fn from(view: &HitPointsRowView) -> Self {
        let HitPointsRowView { value, regen } = view.clone();
        Self { value, regen }
    }
}

impl ddd::Layered for HitPointsRowModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for HitPointsRowModel {
    type View = HitPointsRowView;
}
