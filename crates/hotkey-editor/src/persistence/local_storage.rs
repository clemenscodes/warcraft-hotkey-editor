pub(crate) struct LocalStorage {
    key: &'static str,
}

impl LocalStorage {
    pub(crate) const fn new(key: &'static str) -> Self {
        Self { key }
    }

    pub(crate) fn get(&self) -> Option<String> {
        storage_get(self.key)
    }

    pub(crate) fn set(&self, value: &str) {
        storage_set(self.key, value);
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
