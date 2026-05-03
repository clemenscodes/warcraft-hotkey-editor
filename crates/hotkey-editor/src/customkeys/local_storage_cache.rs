use warcraft_keybinds::CustomKeysFile;

use crate::customkeys::explicit_export::ExplicitExport;
use crate::domain::grid_layout::GridLayout;

const STORAGE_KEY: &str = "warcraft-hotkey-editor.custom-keys";

// The grid layout lives in its own storage entry independent of the
// CustomKeys export so importing a CustomKeys file or applying a template
// can never mutate the user's chosen letter-to-position mapping. The only
// path that updates this entry is the layout editor dialog.
const GRID_LAYOUT_STORAGE_KEY: &str = "warcraft-hotkey-editor.grid-layout";

pub(crate) struct LocalStorageCache;

impl LocalStorageCache {
    pub(crate) fn load() -> Option<CustomKeysFile> {
        storage_get(STORAGE_KEY).map(|contents| CustomKeysFile::from(contents.as_str()))
    }

    pub(crate) fn save_export(file: &CustomKeysFile) {
        let contents = ExplicitExport::serialize(file);
        storage_set(STORAGE_KEY, &contents);
    }

    pub(crate) fn load_grid_layout() -> Option<GridLayout> {
        let raw_value = storage_get(GRID_LAYOUT_STORAGE_KEY)?;
        GridLayout::from_storage_string(&raw_value)
    }

    pub(crate) fn save_grid_layout(layout: GridLayout) {
        let contents = layout.to_storage_string();
        storage_set(GRID_LAYOUT_STORAGE_KEY, &contents);
    }
}

#[cfg(target_arch = "wasm32")]
fn storage_get(key: &str) -> Option<String> {
    web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item(key).ok().flatten())
}

#[cfg(not(target_arch = "wasm32"))]
fn storage_get(_key: &str) -> Option<String> {
    None
}

#[cfg(target_arch = "wasm32")]
fn storage_set(key: &str, value: &str) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        let _ = storage.set_item(key, value);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn storage_set(_key: &str, _value: &str) {}
