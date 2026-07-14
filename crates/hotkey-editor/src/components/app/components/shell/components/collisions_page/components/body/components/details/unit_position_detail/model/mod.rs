use super::view::UnitPositionDetailView;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionUnitView;
use dioxus::prelude::*;

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
