use base64::Engine;
use flate2::Compression;
use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use std::io::Read;
use std::io::Write;

/// A string codec that deflates and base64-encodes text before storage and
/// reverses it on load. Meant for highly repetitive payloads (e.g. a
/// materialized undo timeline) that would otherwise strain a `localStorage`
/// quota. `decode` returns `None` for anything it cannot decode, so a corrupt
/// or foreign blob degrades to "nothing stored".
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct CompressedText;

impl CompressedText {
    pub fn encode(text: &str) -> String {
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
        let _ = encoder.write_all(text.as_bytes());
        let compressed_bytes = encoder.finish().unwrap_or_default();
        base64::engine::general_purpose::STANDARD.encode(compressed_bytes)
    }

    pub fn decode(encoded: &str) -> Option<String> {
        let compressed_bytes = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .ok()?;
        let mut decoder = DeflateDecoder::new(compressed_bytes.as_slice());
        let mut decompressed = String::new();
        decoder.read_to_string(&mut decompressed).ok()?;
        Some(decompressed)
    }
}

#[cfg(test)]
mod tests {
    use super::CompressedText;

    #[test]
    fn round_trips_repetitive_text() {
        let original = "Buttonpos=0,0\n".repeat(500);
        let encoded = CompressedText::encode(&original);
        assert!(encoded.len() < original.len());
        let decoded = CompressedText::decode(&encoded);
        assert_eq!(decoded.as_deref(), Some(original.as_str()));
    }

    #[test]
    fn decode_rejects_a_non_blob() {
        assert_eq!(CompressedText::decode("not base64!!"), None);
    }
}
