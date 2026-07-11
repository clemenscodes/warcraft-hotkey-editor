use warcraft_api::HitPoints;

/// The published `View` contract mirroring [`HitPointsValueModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HitPointsValueView {
    pub value: HitPoints,
}

impl ddd::View for HitPointsValueView {}
