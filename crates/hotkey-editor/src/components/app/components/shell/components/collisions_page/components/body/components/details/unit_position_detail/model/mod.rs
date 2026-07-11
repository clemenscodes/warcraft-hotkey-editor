use super::view::UnitPositionDetailView;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionUnitView;
use dioxus::prelude::*;

/// The per-unit position-collision detail pane: the clashing units. The selected unit
/// and the navigation its links use are read from context, so only the unit list is a
/// prop.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionDetailModel {
    pub units: Vec<UnitPositionUnitView>,
}

impl From<&UnitPositionDetailView> for UnitPositionDetailModel {
    fn from(view: &UnitPositionDetailView) -> Self {
        let UnitPositionDetailView { units } = view.clone();
        Self { units }
    }
}

impl ddd::Model for UnitPositionDetailModel {
    type View = UnitPositionDetailView;
}
