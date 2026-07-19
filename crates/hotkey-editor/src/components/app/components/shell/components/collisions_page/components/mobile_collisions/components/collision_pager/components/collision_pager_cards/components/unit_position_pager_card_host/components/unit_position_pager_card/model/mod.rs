use super::view::UnitPositionPagerCardView;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionUnitView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionPagerCardModel {
    pub unit: UnitPositionUnitView,
}

impl From<&UnitPositionPagerCardView> for UnitPositionPagerCardModel {
    fn from(view: &UnitPositionPagerCardView) -> Self {
        let UnitPositionPagerCardView { unit } = view.clone();
        Self { unit }
    }
}

impl ddd::Model for UnitPositionPagerCardModel {
    type View = UnitPositionPagerCardView;
}
