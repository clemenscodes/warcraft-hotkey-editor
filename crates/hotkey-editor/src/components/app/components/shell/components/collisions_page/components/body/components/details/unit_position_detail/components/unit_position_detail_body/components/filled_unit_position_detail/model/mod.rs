use super::view::FilledUnitPositionDetailView;
use crate::components::app::components::shell::components::collisions_page::presentation::UnitPositionUnitView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FilledUnitPositionDetailModel {
    pub unit_view: UnitPositionUnitView,
}

impl From<&FilledUnitPositionDetailView> for FilledUnitPositionDetailModel {
    fn from(view: &FilledUnitPositionDetailView) -> Self {
        let FilledUnitPositionDetailView { unit_view } = view.clone();
        Self { unit_view }
    }
}

impl ddd::Model for FilledUnitPositionDetailModel {
    type View = FilledUnitPositionDetailView;
}
