use warcraft_api::DefenseType;

/// The published `View` contract mirroring [`DefenseMatchupRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DefenseMatchupRowView {
    pub defense_type: DefenseType,
}

impl ddd::View for DefenseMatchupRowView {}
