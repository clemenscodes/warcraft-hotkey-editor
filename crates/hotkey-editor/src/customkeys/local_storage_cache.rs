use warcraft_keybinds::CustomKeysFile;

use crate::customkeys::explicit_export::ExplicitExport;

const STORAGE_KEY: &str = "warcraft-hotkey-editor.custom-keys";

pub(crate) struct LocalStorageCache;

impl LocalStorageCache {
    pub(crate) fn load() -> Option<CustomKeysFile> {
        storage_get(STORAGE_KEY).map(|contents| CustomKeysFile::from(contents.as_str()))
    }

    pub(crate) fn save_export(file: &CustomKeysFile) {
        let contents = ExplicitExport::serialize(file);
        storage_set(STORAGE_KEY, &contents);
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
