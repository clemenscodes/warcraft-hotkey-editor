use warcraft_keybinds::AttackRange;

/// The published `View` contract mirroring [`RangeRowProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct RangeRowView {
    pub value: AttackRange,
}

impl ddd::View for RangeRowView {}
