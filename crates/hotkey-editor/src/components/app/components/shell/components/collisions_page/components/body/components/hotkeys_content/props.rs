use crate::components::app::components::shell::components::collisions_page::components::body::components::details::hotkey_unit_detail::HotkeyUnitDetailProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::unit_cards_sidebar::UnitCardsSidebarProps;
use crate::components::app::components::shell::components::collisions_page::logic::HotkeyConflictView;
use dioxus::prelude::*;

/// The shared-hotkey two-pane content: the clashing-units sidebar beside the hotkey
/// unit detail pane.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeysContentProps {
    pub sidebar: UnitCardsSidebarProps<HotkeyConflictView>,
    pub detail: HotkeyUnitDetailProps,
}
