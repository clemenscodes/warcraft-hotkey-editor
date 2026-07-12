use super::view::HotkeyUnitDetailBodyView;
use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;
use dioxus::prelude::*;

/// The shared-hotkey detail card's body region input: the clashing units. The selected unit
/// and the navigation its links use are read from context, so only the unit list is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyUnitDetailBodyModel {
    pub units: Vec<HotkeyUnitView>,
}

impl From<&HotkeyUnitDetailBodyView> for HotkeyUnitDetailBodyModel {
    fn from(view: &HotkeyUnitDetailBodyView) -> Self {
        let HotkeyUnitDetailBodyView { units } = view.clone();
        Self { units }
    }
}

impl ddd::Model for HotkeyUnitDetailBodyModel {
    type View = HotkeyUnitDetailBodyView;
}
