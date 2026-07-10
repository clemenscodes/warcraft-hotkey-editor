use super::view::UnitPositionDetailView;
use crate::components::app::components::shell::components::collisions_page::logic::UnitPositionUnitView;
use dioxus::prelude::*;

/// The per-unit position-collision detail pane: the clashing units. The selected unit
/// and the navigation its links use are read from context, so only the unit list is a
/// prop.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionDetailProps {
    pub units: Vec<UnitPositionUnitView>,
}

impl From<&UnitPositionDetailView> for UnitPositionDetailProps {
    fn from(view: &UnitPositionDetailView) -> Self {
        let UnitPositionDetailView { units } = view.clone();
        Self { units }
    }
}

impl ddd::Props for UnitPositionDetailProps {
    type View = UnitPositionDetailView;
}
