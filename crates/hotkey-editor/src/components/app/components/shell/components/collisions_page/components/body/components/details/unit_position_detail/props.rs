use crate::components::app::components::shell::components::collisions_page::logic::UnitPositionUnitView;
use dioxus::prelude::*;

/// The per-unit position-collision detail pane: the clashing units. The selected unit
/// and the navigation its links use are read from context, so only the unit list is a
/// prop.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionDetailProps {
    pub units: Vec<UnitPositionUnitView>,
}
