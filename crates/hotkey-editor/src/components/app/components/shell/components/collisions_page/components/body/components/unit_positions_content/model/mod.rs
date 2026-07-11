use super::view::UnitPositionsContentView;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionUnitView;
use dioxus::prelude::*;

/// The per-unit position-collision two-pane content: the clashing units the sidebar and
/// the unit position detail pane both render.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionsContentModel {
    pub units: Vec<UnitPositionUnitView>,
}

impl From<&UnitPositionsContentView> for UnitPositionsContentModel {
    fn from(view: &UnitPositionsContentView) -> Self {
        let UnitPositionsContentView { units } = view.clone();
        Self { units }
    }
}

impl ddd::Model for UnitPositionsContentModel {
    type View = UnitPositionsContentView;
}
