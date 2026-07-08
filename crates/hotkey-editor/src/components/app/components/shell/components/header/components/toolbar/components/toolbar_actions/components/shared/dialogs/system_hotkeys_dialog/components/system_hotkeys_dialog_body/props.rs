use super::components::inventory_drag_overlay::InventoryDragOverlayProps;
use super::components::system_hotkeys_body::SystemHotkeysBodyProps;
use super::components::system_hotkeys_breadcrumbs::SystemHotkeysBreadcrumbsProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryDragFollower;
use dioxus::prelude::*;
use warcraft_database::SystemHotkeysCategory;
use warcraft_keybinds::WarcraftObjectId;

/// The system-hotkeys dialog's scroll region inputs: the active category tab, the
/// shared editing-section signal, and the inventory drag follower. Each child's
/// props are derived from these by conversion.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysDialogBodyProps {
    pub active_category: Signal<SystemHotkeysCategory>,
    pub editing_section: Signal<Option<WarcraftObjectId>>,
    pub drag_follower: Signal<Option<InventoryDragFollower>>,
}

impl From<&SystemHotkeysDialogBodyProps> for SystemHotkeysBreadcrumbsProps {
    fn from(props: &SystemHotkeysDialogBodyProps) -> Self {
        let active_category = props.active_category;
        Self { active_category }
    }
}

impl From<&SystemHotkeysDialogBodyProps> for SystemHotkeysBodyProps {
    fn from(props: &SystemHotkeysDialogBodyProps) -> Self {
        let active_category = props.active_category;
        let editing_section = props.editing_section;
        let drag_follower = props.drag_follower;
        Self {
            active_category,
            editing_section,
            drag_follower,
        }
    }
}

impl From<&SystemHotkeysDialogBodyProps> for InventoryDragOverlayProps {
    fn from(props: &SystemHotkeysDialogBodyProps) -> Self {
        let drag_follower = props.drag_follower;
        Self { drag_follower }
    }
}
