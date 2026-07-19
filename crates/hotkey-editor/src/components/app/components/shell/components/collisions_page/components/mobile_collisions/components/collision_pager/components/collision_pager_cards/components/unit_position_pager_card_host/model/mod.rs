use super::view::UnitPositionPagerCardHostView;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionUnitView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionPagerCardHostModel {
    pub unit: UnitPositionUnitView,
}

impl From<&UnitPositionPagerCardHostView> for UnitPositionPagerCardHostModel {
    fn from(view: &UnitPositionPagerCardHostView) -> Self {
        let UnitPositionPagerCardHostView { unit } = view.clone();
        Self { unit }
    }
}

impl ddd::Model for UnitPositionPagerCardHostModel {
    type View = UnitPositionPagerCardHostView;
}
