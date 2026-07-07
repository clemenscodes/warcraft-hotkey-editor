use super::props::InventoryHotkeysViewProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::{
    InventoryGrid, InventoryGridProps,
};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_hotkeys_section::SystemHotkeysSectionProps;
use dioxus::prelude::*;

impl From<&InventoryHotkeysViewProps> for InventoryGridProps {
    fn from(props: &InventoryHotkeysViewProps) -> Self {
        let editing_section = props.editing_section;
        let drag_follower = props.drag_follower;
        Self {
            editing_section,
            drag_follower,
        }
    }
}

impl From<&InventoryHotkeysViewProps> for SystemHotkeysSectionProps {
    fn from(props: &InventoryHotkeysViewProps) -> Self {
        let intro = String::from("Drag a slot onto another to swap their keys.");
        let grid = InventoryGridProps::from(props);
        let children = rsx! {
            InventoryGrid { ..grid }
        };
        Self { intro, children }
    }
}
