use crate::components::app::components::shell::components::collisions_page::logic::UnitPositionUnitView;
use dioxus::prelude::*;

/// The per-unit position-collision two-pane content: the clashing units the sidebar and
/// the unit position detail pane both render.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionsContentProps {
    pub units: Vec<UnitPositionUnitView>,
}
