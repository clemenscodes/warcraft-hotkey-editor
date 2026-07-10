use super::view::InventorySlotView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::components::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryFilledSlotEntry;
use dioxus::prelude::*;

/// One grid position: the filled cell's raw values when the slot is occupied, or
/// `None` for an empty position (which renders the placeholder).
#[derive(Props, Clone, PartialEq)]
pub struct InventorySlotProps {
    pub(crate) filled: Option<InventoryFilledSlotEntry>,
}

impl From<&InventorySlotView> for InventorySlotProps {
    fn from(view: &InventorySlotView) -> Self {
        let InventorySlotView { filled } = view.clone();
        Self { filled }
    }
}

impl ddd::Props for InventorySlotProps {
    type View = InventorySlotView;
}
