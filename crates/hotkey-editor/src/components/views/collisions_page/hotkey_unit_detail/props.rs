use crate::components::views::collisions_page::HotkeyUnitView;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// The hotkey-collision detail pane: the clashing units, the selected one, and the
/// navigation context its links use.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyUnitDetailProps {
    pub units: Vec<HotkeyUnitView>,
    pub selected_unit: Signal<Option<String>>,
    pub view_navigation: ViewNavigationContext,
}
