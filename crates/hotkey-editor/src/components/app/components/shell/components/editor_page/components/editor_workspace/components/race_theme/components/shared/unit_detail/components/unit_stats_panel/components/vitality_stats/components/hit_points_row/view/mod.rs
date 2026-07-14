use warcraft_api::HitPoints;

#[derive(Clone, PartialEq)]
pub struct HitPointsRowView {
    pub value: HitPoints,
}

impl ddd::View for HitPointsRowView {}
