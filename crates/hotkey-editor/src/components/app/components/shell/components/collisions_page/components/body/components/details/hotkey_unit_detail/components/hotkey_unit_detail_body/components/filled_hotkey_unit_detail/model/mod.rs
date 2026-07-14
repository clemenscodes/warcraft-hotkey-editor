use super::view::FilledHotkeyUnitDetailView;
use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FilledHotkeyUnitDetailModel {
    pub unit_view: HotkeyUnitView,
}

impl From<&FilledHotkeyUnitDetailView> for FilledHotkeyUnitDetailModel {
    fn from(view: &FilledHotkeyUnitDetailView) -> Self {
        let FilledHotkeyUnitDetailView { unit_view } = view.clone();
        Self { unit_view }
    }
}

impl ddd::Model for FilledHotkeyUnitDetailModel {
    type View = FilledHotkeyUnitDetailView;
}
