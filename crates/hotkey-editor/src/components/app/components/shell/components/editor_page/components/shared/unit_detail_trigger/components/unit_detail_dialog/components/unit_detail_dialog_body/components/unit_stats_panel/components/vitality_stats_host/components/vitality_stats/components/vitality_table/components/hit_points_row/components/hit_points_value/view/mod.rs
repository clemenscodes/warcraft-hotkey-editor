use warcraft_api::HitPoints;

#[derive(Clone, PartialEq)]
pub struct HitPointsValueView {
    pub value: HitPoints,
}

impl ddd::Layered for HitPointsValueView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HitPointsValueView {}
