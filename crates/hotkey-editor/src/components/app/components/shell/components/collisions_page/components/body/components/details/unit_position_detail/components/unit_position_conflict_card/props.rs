use crate::components::app::components::shell::components::collisions_page::logic::UnitPositionConflictView;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// One position conflict on a unit: the clashing abilities, the owning unit key the
/// ability icons deep-link through, and the navigation context.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionConflictCardProps {
    pub conflict: UnitPositionConflictView,
    #[props(into)]
    pub unit_id: String,
    pub view_navigation: ViewNavigationContext,
}
