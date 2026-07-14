use super::view::InventorySlotView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryFilledSlotEntry;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InventorySlotModel {
    pub(crate) filled: Option<InventoryFilledSlotEntry>,
}

impl From<&InventorySlotView> for InventorySlotModel {
    fn from(view: &InventorySlotView) -> Self {
        let InventorySlotView { filled } = view.clone();
        Self { filled }
    }
}

impl ddd::Model for InventorySlotModel {
    type View = InventorySlotView;
}
