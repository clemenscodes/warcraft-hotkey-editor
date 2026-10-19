pub(crate) fn trigger(filename: &str, contents: &str) {
    browser_kit::dom::BlobDownload::text(filename, contents);
}
