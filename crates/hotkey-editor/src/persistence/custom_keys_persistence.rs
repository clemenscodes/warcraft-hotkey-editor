use browser_kit::storage::LocalStorage;

const CUSTOM_KEYS_STORAGE: LocalStorage = LocalStorage::new("warcraft-hotkey-editor.custom-keys");

pub(crate) struct CustomKeysPersistence;

impl CustomKeysPersistence {
    pub(crate) fn load_text() -> Option<String> {
        CUSTOM_KEYS_STORAGE.get()
    }

    pub(crate) fn save_text(text: &str) {
        CUSTOM_KEYS_STORAGE.set(text);
    }
}
