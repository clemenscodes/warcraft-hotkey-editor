use super::view::HotkeyUnitDetailView;
use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyUnitDetailModel {
    pub units: Vec<HotkeyUnitView>,
}

impl From<&HotkeyUnitDetailView> for HotkeyUnitDetailModel {
    fn from(view: &HotkeyUnitDetailView) -> Self {
        let HotkeyUnitDetailView { units } = view.clone();
        Self { units }
    }
}

impl ddd::Model for HotkeyUnitDetailModel {
    type View = HotkeyUnitDetailView;
}
