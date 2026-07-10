use warcraft_keybinds::HitPointsRegen;

/// The published `View` contract mirroring [`HitPointsRegenRowProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HitPointsRegenRowView {
    pub value: HitPointsRegen,
}

impl ddd::View for HitPointsRegenRowView {}
