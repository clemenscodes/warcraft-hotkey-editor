use browser_kit::storage::LocalStorage;
use warcraft_keybinds::GridLayout;

const GRID_LAYOUT_STORAGE: LocalStorage = LocalStorage::new("warcraft-hotkey-editor.grid-layout");

/// The chosen grid layout, under its own key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(crate) struct GridLayoutPersistence;

impl GridLayoutPersistence {
    pub(crate) fn load() -> Option<GridLayout> {
        let raw_value = GRID_LAYOUT_STORAGE.get()?;
        GridLayout::try_from(raw_value.as_str()).ok()
    }

    pub(crate) fn save(layout: GridLayout) {
        let contents = layout.to_storage_string();
        GRID_LAYOUT_STORAGE.set(&contents);
    }

    pub(crate) fn remove() {
        GRID_LAYOUT_STORAGE.remove();
    }
}
