use browser_kit::storage::LocalStorage;
use warcraft_keybinds::GridLayout;

const GRID_LAYOUT_STORAGE: LocalStorage = LocalStorage::new("warcraft-hotkey-editor.grid-layout");

pub(crate) struct GridLayoutPersistence;

impl GridLayoutPersistence {
    pub(crate) fn load_grid_layout() -> Option<GridLayout> {
        let raw_value = GRID_LAYOUT_STORAGE.get()?;
        GridLayout::try_from(raw_value.as_str()).ok()
    }

    pub(crate) fn save_grid_layout(layout: GridLayout) {
        let contents = layout.to_storage_string();
        GRID_LAYOUT_STORAGE.set(&contents);
    }
}
