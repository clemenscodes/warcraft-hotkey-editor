use warcraft_api::ManaRegen;

/// The published `View` contract mirroring [`ManaRegenRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ManaRegenRowView {
    pub value: ManaRegen,
}

impl ddd::View for ManaRegenRowView {}
