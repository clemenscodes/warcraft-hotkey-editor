use browser_kit::storage::LocalStorage;

const CUSTOM_KEYS_STORAGE: LocalStorage = LocalStorage::new("warcraft-hotkey-editor.custom-keys");

/// The CustomKeys.txt text, the single source of truth, under its own key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(crate) struct CustomKeysPersistence;

impl CustomKeysPersistence {
    pub(crate) fn load() -> Option<String> {
        CUSTOM_KEYS_STORAGE.get()
    }

    pub(crate) fn save(text: &str) {
        CUSTOM_KEYS_STORAGE.set(text);
    }

    pub(crate) fn remove() {
        CUSTOM_KEYS_STORAGE.remove();
    }
}
