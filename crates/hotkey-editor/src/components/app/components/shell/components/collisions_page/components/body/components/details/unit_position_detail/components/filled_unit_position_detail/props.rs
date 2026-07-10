use crate::components::app::components::shell::components::collisions_page::logic::UnitPositionUnitView;
use dioxus::prelude::*;

/// The populated position-collision detail pane: the selected unit's view, whose header
/// and position-conflict cards this pane shapes and renders.
#[derive(Props, Clone, PartialEq)]
pub struct FilledUnitPositionDetailProps {
    pub unit_view: UnitPositionUnitView,
}
