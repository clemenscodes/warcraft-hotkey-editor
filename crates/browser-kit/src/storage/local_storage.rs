/// A single `localStorage` key, read and written as a `String`. On a non-wasm
/// target (native `cargo test`) `get` yields `None` and `set` is a no-op, so a
/// consumer's persistence layer compiles and tests without a browser.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocalStorage {
    key: &'static str,
}

impl LocalStorage {
    pub const fn new(key: &'static str) -> Self {
        Self { key }
    }

    pub fn key(&self) -> &'static str {
        self.key
    }

    pub fn get(&self) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window()?;
            let storage = window.local_storage().ok().flatten()?;
            storage.get_item(self.key).ok().flatten()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            None
        }
    }

    pub fn set(&self, value: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            let stored = web_sys::window().and_then(|window| window.local_storage().ok().flatten());
            if let Some(storage) = stored {
                let _ = storage.set_item(self.key, value);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = value;
        }
    }
}
