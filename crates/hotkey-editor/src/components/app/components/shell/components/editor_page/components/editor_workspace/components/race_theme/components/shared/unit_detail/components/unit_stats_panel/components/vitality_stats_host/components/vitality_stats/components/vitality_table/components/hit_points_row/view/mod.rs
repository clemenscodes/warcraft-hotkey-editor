use warcraft_api::{HitPoints, HitPointsRegen};

#[derive(Clone, PartialEq)]
pub struct HitPointsRowView {
    pub value: HitPoints,
    pub regen: HitPointsRegen,
}

impl ddd::Layered for HitPointsRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HitPointsRowView {}
