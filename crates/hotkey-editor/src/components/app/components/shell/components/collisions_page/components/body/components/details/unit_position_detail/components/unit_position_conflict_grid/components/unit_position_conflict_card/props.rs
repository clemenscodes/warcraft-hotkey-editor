use crate::components::app::components::shell::components::collisions_page::logic::UnitPositionConflictView;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// One position conflict on a unit: the clashing abilities, the owning unit id the
/// ability icons deep-link through, and the navigation context.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionConflictCardProps {
    pub conflict: UnitPositionConflictView,
    pub unit_id: WarcraftObjectId,
    pub view_navigation: ViewNavigationContext,
}
