use crate::grid_layout::GridLayout;

const STORAGE_KEY: &str = "warcraft-hotkey-editor.custom-keys";

const GRID_LAYOUT_STORAGE_KEY: &str = "warcraft-hotkey-editor.grid-layout";

pub(crate) struct LocalStorageCache;

impl LocalStorageCache {
    pub(crate) fn load_text() -> Option<String> {
        storage_get(STORAGE_KEY)
    }

    pub(crate) fn save_text(text: &str) {
        storage_set(STORAGE_KEY, text);
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
