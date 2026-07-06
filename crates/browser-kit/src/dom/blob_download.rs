/// Triggers a browser download of in-memory text as a file, via a temporary
/// blob URL and a synthetic `<a download>` click.
pub struct BlobDownload;

impl BlobDownload {
    /// Download `contents` as a UTF-8 `text/plain` file named `filename`.
    pub fn text(filename: &str, contents: &str) {
        Self::typed(filename, contents, "text/plain;charset=utf-8");
    }

    /// Download `contents` as `filename` with an explicit MIME type.
    pub fn typed(filename: &str, contents: &str, mime_type: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            let parts = js_sys::Array::new();
            let contents_value = wasm_bindgen::JsValue::from_str(contents);
            parts.push(&contents_value);
            let property_bag = web_sys::BlobPropertyBag::new();
            property_bag.set_type(mime_type);
            let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&parts, &property_bag)
            else {
                return;
            };
            let Ok(blob_url) = web_sys::Url::create_object_url_with_blob(&blob) else {
                return;
            };
            let created = web_sys::window().and_then(|window| window.document());
            let Some(document) = created else {
                let _ = web_sys::Url::revoke_object_url(&blob_url);
                return;
            };
            let anchor = document
                .create_element("a")
                .ok()
                .and_then(|element| element.dyn_into::<web_sys::HtmlAnchorElement>().ok());
            if let Some(anchor) = anchor {
                anchor.set_href(&blob_url);
                anchor.set_download(filename);
                anchor.click();
            }
            let _ = web_sys::Url::revoke_object_url(&blob_url);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (filename, contents, mime_type);
        }
    }
}
