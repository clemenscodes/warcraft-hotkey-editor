use browser_kit::storage::CompressedText;
use browser_kit::storage::LocalStorage;

const UNDO_STORAGE: LocalStorage = LocalStorage::new("warcraft-hotkey-editor.undo-history");

/// The undo history, stored compressed under its own key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(crate) struct EditorHistoryPersistence;

impl EditorHistoryPersistence {
    pub(crate) fn load() -> Option<String> {
        let raw = UNDO_STORAGE.get()?;
        CompressedText::decode(&raw)
    }

    pub(crate) fn save(text: &str) {
        let compressed = CompressedText::encode(text);
        UNDO_STORAGE.set(&compressed);
    }

    pub(crate) fn remove() {
        UNDO_STORAGE.remove();
    }
}
