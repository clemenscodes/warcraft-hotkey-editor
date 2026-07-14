use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryFilledSlotEntry;

#[derive(Clone, PartialEq)]
pub struct InventorySlotView {
    pub(crate) filled: Option<InventoryFilledSlotEntry>,
}

impl ddd::View for InventorySlotView {}
