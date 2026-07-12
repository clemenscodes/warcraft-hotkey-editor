use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionConflictView;
use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`UnitPositionConflictCardModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitPositionConflictCardView {
    pub conflict: UnitPositionConflictView,
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for UnitPositionConflictCardView {}
