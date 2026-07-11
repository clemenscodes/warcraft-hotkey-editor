use super::view::HotkeyUnitDetailView;
use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;
use dioxus::prelude::*;

/// The hotkey-collision detail pane: the clashing units. The selected unit and the
/// navigation its links use are read from context, so only the unit list is a prop.
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
