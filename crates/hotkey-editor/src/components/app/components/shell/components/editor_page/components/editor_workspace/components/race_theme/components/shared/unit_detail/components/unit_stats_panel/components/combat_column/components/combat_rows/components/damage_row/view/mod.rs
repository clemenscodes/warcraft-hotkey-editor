use warcraft_api::DamageRange;

/// The published `View` contract mirroring [`DamageRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DamageRowView {
    pub value: DamageRange,
}

impl ddd::View for DamageRowView {}
