use browser_kit::storage::CompressedText;
use browser_kit::storage::LocalStorage;

const UNDO_STORAGE: LocalStorage = LocalStorage::new("warcraft-hotkey-editor.undo-history");

pub(crate) fn load_text() -> Option<String> {
    let raw = UNDO_STORAGE.get()?;
    CompressedText::decode(&raw)
}

pub(crate) fn save_text(text: &str) {
    let compressed = CompressedText::encode(text);
    UNDO_STORAGE.set(&compressed);
}
