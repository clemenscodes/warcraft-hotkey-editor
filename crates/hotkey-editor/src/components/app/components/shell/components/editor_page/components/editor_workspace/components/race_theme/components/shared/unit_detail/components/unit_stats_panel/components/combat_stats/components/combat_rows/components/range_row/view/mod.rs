use warcraft_api::AttackRange;

/// The published `View` contract mirroring [`RangeRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct RangeRowView {
    pub value: AttackRange,
}

impl ddd::View for RangeRowView {}
