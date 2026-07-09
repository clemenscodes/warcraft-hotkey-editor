use crate::components::app::components::shell::components::collisions_page::logic::HotkeyUnitView;
use dioxus::prelude::*;

/// The hotkey-collision detail pane: the clashing units. The selected unit and the
/// navigation its links use are read from context, so only the unit list is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyUnitDetailProps {
    pub units: Vec<HotkeyUnitView>,
}
