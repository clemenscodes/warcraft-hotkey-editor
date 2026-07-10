use warcraft_keybinds::HitPoints;

/// The published `View` contract mirroring [`HitPointsValueProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HitPointsValueView {
    pub value: HitPoints,
}

impl ddd::View for HitPointsValueView {}
