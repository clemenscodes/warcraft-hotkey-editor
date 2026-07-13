use warcraft_api::SystemHotkeysCategory;

/// The inventory editor's shaped content: the intro caption the domain supplies for the
/// Inventory category.
pub(super) struct InventoryHotkeysViewModel {
    pub(super) caption: &'static str,
}

/// Sources the Inventory category's intro caption from the domain, so the renderer never
/// hardcodes the copy.
pub(super) fn use_inventory_hotkeys_view() -> InventoryHotkeysViewModel {
    let caption = SystemHotkeysCategory::Inventory
        .caption()
        .unwrap_or_default();
    InventoryHotkeysViewModel { caption }
}
