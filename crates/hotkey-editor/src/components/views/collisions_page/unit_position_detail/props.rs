use crate::components::views::collisions_page::UnitPositionUnitView;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// The per-unit position-collision detail pane: the clashing units, the selected
/// one, and the navigation context its links use.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionDetailProps {
    pub units: Vec<UnitPositionUnitView>,
    pub selected_unit: Signal<Option<String>>,
    pub view_navigation: ViewNavigationContext,
}
