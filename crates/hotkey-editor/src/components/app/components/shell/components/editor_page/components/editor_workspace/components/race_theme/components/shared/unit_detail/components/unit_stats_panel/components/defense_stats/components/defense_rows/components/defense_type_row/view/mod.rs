use warcraft_api::DefenseType;

/// The published `View` contract mirroring [`DefenseTypeRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DefenseTypeRowView {
    pub value: DefenseType,
}

impl ddd::View for DefenseTypeRowView {}
