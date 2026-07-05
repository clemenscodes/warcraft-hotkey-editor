use crate::persistence::local_storage::LocalStorage;

const UNDO_STORAGE: LocalStorage = LocalStorage::new("warcraft-hotkey-editor.undo-history");

pub(crate) struct EditorHistoryPersistence;

impl EditorHistoryPersistence {
    /// Returns the decompressed canonical history text, or `None` when nothing is
    /// stored or the blob is unreadable.
    pub(crate) fn load_text() -> Option<String> {
        let raw = UNDO_STORAGE.get()?;
        decompress_blob(&raw)
    }

    /// Compresses and stores the canonical history text. The materialized text is
    /// highly repetitive, so deflate keeps a deep history inside localStorage.
    pub(crate) fn save_text(text: &str) {
        let compressed = compress_blob(text);
        UNDO_STORAGE.set(&compressed);
    }
}

fn compress_blob(text: &str) -> String {
    use base64::Engine;
    use flate2::Compression;
    use flate2::write::DeflateEncoder;
    use std::io::Write;
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    let _ = encoder.write_all(text.as_bytes());
    let compressed_bytes = encoder.finish().unwrap_or_default();
    base64::engine::general_purpose::STANDARD.encode(compressed_bytes)
}

fn decompress_blob(encoded: &str) -> Option<String> {
    use base64::Engine;
    use flate2::read::DeflateDecoder;
    use std::io::Read;
    let compressed_bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .ok()?;
    let mut decoder = DeflateDecoder::new(compressed_bytes.as_slice());
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed).ok()?;
    Some(decompressed)
}
