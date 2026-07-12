use warcraft_api::DamagePerSecond;

/// The published `View` contract mirroring [`DamagePerSecondRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DamagePerSecondRowView {
    pub value: Option<DamagePerSecond>,
}

impl ddd::View for DamagePerSecondRowView {}
