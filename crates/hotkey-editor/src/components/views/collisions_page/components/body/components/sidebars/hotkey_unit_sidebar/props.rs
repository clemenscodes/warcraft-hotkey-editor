use crate::components::views::collisions_page::logic::HotkeyUnitView;
use dioxus::prelude::*;

/// The unit sidebar: the clashing units and the selected key it drives.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyUnitSidebarProps {
    pub units: Vec<HotkeyUnitView>,
    pub selected_unit: Signal<Option<String>>,
}
