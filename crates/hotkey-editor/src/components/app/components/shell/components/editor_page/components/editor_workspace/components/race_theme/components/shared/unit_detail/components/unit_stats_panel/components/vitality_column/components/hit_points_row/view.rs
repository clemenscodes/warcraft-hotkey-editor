use warcraft_keybinds::HitPoints;

/// The published `View` contract mirroring [`HitPointsRowProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HitPointsRowView {
    pub value: HitPoints,
}

impl ddd::View for HitPointsRowView {}
