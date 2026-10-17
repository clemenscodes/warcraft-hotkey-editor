use warcraft_api::AttackRange;

#[derive(Clone, PartialEq)]
pub struct RangeRowView {
    pub value: AttackRange,
}

impl ddd::Layered for RangeRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RangeRowView {}
