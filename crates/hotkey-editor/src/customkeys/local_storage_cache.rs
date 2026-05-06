use warcraft_keybinds::CustomKeys;

use crate::domain::grid_layout::GridLayout;

const STORAGE_KEY: &str = "warcraft-hotkey-editor.custom-keys";

// The grid layout lives in its own storage entry independent of the
// CustomKeys export so importing a CustomKeys file or applying a template
// can never mutate the user's chosen letter-to-position mapping. The only
// path that updates this entry is the layout editor dialog.
const GRID_LAYOUT_STORAGE_KEY: &str = "warcraft-hotkey-editor.grid-layout";

pub(crate) struct LocalStorageCache;

impl LocalStorageCache {
    /// Read the canonical CustomKeys.txt text from localStorage.
    /// Returns `None` on first boot before anything has been persisted.
    pub(crate) fn load_text() -> Option<String> {
        storage_get(STORAGE_KEY)
    }

    /// Persist a `CustomKeys` instance to localStorage as its canonical
    /// normalized text. This is the only legal write path — every
    /// mutation in the app routes through here.
    pub(crate) fn save_custom_keys(custom_keys: &CustomKeys) {
        let canonical_text = custom_keys.to_text();
        storage_set(STORAGE_KEY, canonical_text);
    }

    pub(crate) fn load_grid_layout() -> Option<GridLayout> {
        let raw_value = storage_get(GRID_LAYOUT_STORAGE_KEY)?;
        GridLayout::try_from(raw_value.as_str()).ok()
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
