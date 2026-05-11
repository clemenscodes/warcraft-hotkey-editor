use crate::model::grid::GridLayout;
use crate::services::storage::local_storage::LocalStorage;

const CUSTOM_KEYS_STORAGE: LocalStorage = LocalStorage::new("warcraft-hotkey-editor.custom-keys");
const GRID_LAYOUT_STORAGE: LocalStorage = LocalStorage::new("warcraft-hotkey-editor.grid-layout");

pub(crate) struct CustomKeysPersistence;

impl CustomKeysPersistence {
    pub(crate) fn load_text() -> Option<String> {
        CUSTOM_KEYS_STORAGE.get()
    }

    pub(crate) fn save_text(text: &str) {
        CUSTOM_KEYS_STORAGE.set(text);
    }

    pub(crate) fn load_grid_layout() -> Option<GridLayout> {
        let raw_value = GRID_LAYOUT_STORAGE.get()?;
        GridLayout::try_from(raw_value.as_str()).ok()
    }

    pub(crate) fn save_grid_layout(layout: GridLayout) {
        let contents = layout.to_storage_string();
        GRID_LAYOUT_STORAGE.set(&contents);
    }
}
