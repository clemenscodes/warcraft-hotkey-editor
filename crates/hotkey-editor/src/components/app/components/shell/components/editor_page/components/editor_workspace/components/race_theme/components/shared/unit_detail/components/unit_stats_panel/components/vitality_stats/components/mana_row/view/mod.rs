use warcraft_api::Mana;

/// The published `View` contract mirroring [`ManaRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ManaRowView {
    pub value: Mana,
}

impl ddd::View for ManaRowView {}
