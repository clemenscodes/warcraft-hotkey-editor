use super::props::InventoryHotkeysViewProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_hotkeys_section_intro::SystemHotkeysSectionIntroProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryGridProps;

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

impl From<&InventoryHotkeysViewProps> for SystemHotkeysSectionIntroProps {
    fn from(_props: &InventoryHotkeysViewProps) -> Self {
        let text = String::from("Drag a slot onto another to swap their keys.");
        Self { text }
    }
}
