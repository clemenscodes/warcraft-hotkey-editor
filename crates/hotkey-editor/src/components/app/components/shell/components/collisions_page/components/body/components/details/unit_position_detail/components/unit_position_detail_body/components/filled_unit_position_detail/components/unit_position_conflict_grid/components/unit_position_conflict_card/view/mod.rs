use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionConflictView;
use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct UnitPositionConflictCardView {
    pub conflict: UnitPositionConflictView,
    pub unit_id: WarcraftObjectId,
}

impl ddd::Layered for UnitPositionConflictCardView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitPositionConflictCardView {}
