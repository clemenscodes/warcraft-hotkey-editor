/// The app's file-download seam: exports the current keys as a `.txt`. The
/// browser mechanics live in `browser_kit::dom::BlobDownload`.
pub(crate) struct BlobDownload;

impl BlobDownload {
    pub(crate) fn trigger(filename: &str, contents: &str) {
        browser_kit::dom::BlobDownload::text(filename, contents);
    }
}
