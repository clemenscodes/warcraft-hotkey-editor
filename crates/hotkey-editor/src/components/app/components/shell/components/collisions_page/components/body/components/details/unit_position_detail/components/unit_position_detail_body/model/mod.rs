use super::view::UnitPositionDetailBodyView;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionUnitView;
use dioxus::prelude::*;

/// The per-unit position-collision detail card's body region input: the clashing units. The
/// selected unit and the navigation its links use are read from context, so only the unit
/// list is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionDetailBodyModel {
    pub units: Vec<UnitPositionUnitView>,
}

impl From<&UnitPositionDetailBodyView> for UnitPositionDetailBodyModel {
    fn from(view: &UnitPositionDetailBodyView) -> Self {
        let UnitPositionDetailBodyView { units } = view.clone();
        Self { units }
    }
}

impl ddd::Model for UnitPositionDetailBodyModel {
    type View = UnitPositionDetailBodyView;
}
