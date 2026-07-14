use super::view::UnitPositionDetailBodyView;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionUnitView;
use dioxus::prelude::*;

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
