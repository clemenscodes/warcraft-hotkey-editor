use warcraft_api::AttackType;

/// The published `View` contract mirroring [`AttackTypeRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AttackTypeRowView {
    pub value: AttackType,
}

impl ddd::View for AttackTypeRowView {}
