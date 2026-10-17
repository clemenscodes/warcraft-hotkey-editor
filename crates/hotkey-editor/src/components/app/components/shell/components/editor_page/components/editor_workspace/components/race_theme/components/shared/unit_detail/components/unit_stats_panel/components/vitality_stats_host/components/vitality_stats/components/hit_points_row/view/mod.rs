use warcraft_api::HitPoints;

#[derive(Clone, PartialEq)]
pub struct HitPointsRowView {
    pub value: HitPoints,
}

impl ddd::Layered for HitPointsRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HitPointsRowView {}
