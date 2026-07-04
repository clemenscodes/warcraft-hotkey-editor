use super::components::inventory_drag_overlay::{InventoryDragOverlay, InventoryDragOverlayProps};
use super::components::system_hotkeys_body::{SystemHotkeysBody, SystemHotkeysBodyProps};
use super::components::system_hotkeys_breadcrumbs::{
    SystemHotkeysBreadcrumbs, SystemHotkeysBreadcrumbsProps,
};
use super::hooks::SystemHotkeysDialogModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::dialog::DialogProps;
use dioxus::prelude::*;

impl From<&SystemHotkeysDialogModel> for DialogProps {
    fn from(model: &SystemHotkeysDialogModel) -> Self {
        let open = model.open;
        let title = String::from("System Hotkeys");
        let breadcrumbs = SystemHotkeysBreadcrumbsProps {
            active_category: model.active_category,
        };
        let body = SystemHotkeysBodyProps {
            active_category: model.active_category,
            loaded_keys: model.loaded_keys,
            editing_section: model.editing_section,
            drag_follower: model.drag_follower,
        };
        let overlay = InventoryDragOverlayProps {
            drag_follower: model.drag_follower,
        };
        let children = rsx! {
            SystemHotkeysBreadcrumbs { ..breadcrumbs }
            SystemHotkeysBody { ..body }
            InventoryDragOverlay { ..overlay }
        };
        Self {
            open,
            title,
            children,
            footer: None,
            on_open_change: None,
        }
    }
}
