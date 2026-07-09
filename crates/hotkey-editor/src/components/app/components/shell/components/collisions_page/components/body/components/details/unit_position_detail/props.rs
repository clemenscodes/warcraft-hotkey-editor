use crate::components::app::components::shell::components::collisions_page::logic::UnitPositionUnitView;
use dioxus::prelude::*;

/// The per-unit position-collision detail pane: the clashing units and the selected
/// one. Its links read the navigation from context, so no navigation is threaded here.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionDetailProps {
    pub units: Vec<UnitPositionUnitView>,
    pub selected_unit: Signal<Option<String>>,
}
