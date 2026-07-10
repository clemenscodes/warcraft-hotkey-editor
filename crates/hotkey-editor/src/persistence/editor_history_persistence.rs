use browser_kit::storage::CompressedText;
use browser_kit::storage::LocalStorage;

const UNDO_STORAGE: LocalStorage = LocalStorage::new("warcraft-hotkey-editor.undo-history");

/// Returns the decompressed canonical history text, or `None` when nothing is
/// stored or the blob is unreadable.
pub(crate) fn load_text() -> Option<String> {
    let raw = UNDO_STORAGE.get()?;
    CompressedText::decode(&raw)
}

/// Compresses and stores the canonical history text. The materialized text is
/// highly repetitive, so deflate keeps a deep history inside localStorage.
pub(crate) fn save_text(text: &str) {
    let compressed = CompressedText::encode(text);
    UNDO_STORAGE.set(&compressed);
}
