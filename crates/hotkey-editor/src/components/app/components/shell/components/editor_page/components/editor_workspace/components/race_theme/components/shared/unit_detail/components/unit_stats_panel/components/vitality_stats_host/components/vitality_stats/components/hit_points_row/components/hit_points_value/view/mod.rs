use warcraft_api::HitPoints;

#[derive(Clone, PartialEq)]
pub struct HitPointsValueView {
    pub value: HitPoints,
}

impl ddd::View for HitPointsValueView {}
