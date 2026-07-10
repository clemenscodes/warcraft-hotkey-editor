use crate::components::app::components::shell::components::collisions_page::logic::HotkeyUnitView;
use dioxus::prelude::*;

/// The populated shared-hotkey detail pane: the selected unit's view, whose header and
/// shared-hotkey conflict cards this pane shapes and renders.
#[derive(Props, Clone, PartialEq)]
pub struct FilledHotkeyUnitDetailProps {
    pub unit_view: HotkeyUnitView,
}
