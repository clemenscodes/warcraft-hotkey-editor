use crate::components::app::components::shell::components::collisions_page::logic::HotkeyUnitView;
use dioxus::prelude::*;

/// The hotkey-collision detail pane: the clashing units and the selected one. Its links
/// read the navigation from context, so no navigation is threaded here.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyUnitDetailProps {
    pub units: Vec<HotkeyUnitView>,
    pub selected_unit: Signal<Option<String>>,
}
