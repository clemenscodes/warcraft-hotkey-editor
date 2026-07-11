use warcraft_api::HitPoints;

/// The published `View` contract mirroring [`HitPointsRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HitPointsRowView {
    pub value: HitPoints,
}

impl ddd::View for HitPointsRowView {}
