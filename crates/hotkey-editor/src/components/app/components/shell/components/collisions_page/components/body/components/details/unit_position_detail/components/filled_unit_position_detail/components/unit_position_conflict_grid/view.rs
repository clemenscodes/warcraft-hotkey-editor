use crate::components::app::components::shell::components::collisions_page::logic::UnitPositionConflictView;
use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`UnitPositionConflictGridProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitPositionConflictGridView {
    pub conflicts: Vec<UnitPositionConflictView>,
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for UnitPositionConflictGridView {}
