use warcraft_api::SystemHotkeysCategory;

pub(super) struct InventoryHotkeysViewModel {
    pub(super) caption: &'static str,
}

pub(super) fn use_inventory_hotkeys_view() -> InventoryHotkeysViewModel {
    let caption = SystemHotkeysCategory::Inventory
        .caption()
        .unwrap_or_default();
    InventoryHotkeysViewModel { caption }
}
