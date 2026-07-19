use warcraft_api::AttackRange;

#[derive(Clone, PartialEq)]
pub struct RangeRowView {
    pub value: AttackRange,
}

impl ddd::View for RangeRowView {}
