use wasm_bindgen::JsCast;

pub(crate) struct BlobDownload;

impl BlobDownload {
    pub(crate) fn trigger(filename: &str, contents: &str) {
        let parts = js_sys::Array::new();
        parts.push(&wasm_bindgen::JsValue::from_str(contents));
        let property_bag = web_sys::BlobPropertyBag::new();
        property_bag.set_type("text/plain;charset=utf-8");
        let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&parts, &property_bag)
        else {
            return;
        };
        let Ok(blob_url) = web_sys::Url::create_object_url_with_blob(&blob) else {
            return;
        };
        let Some(window) = web_sys::window() else {
            let _ = web_sys::Url::revoke_object_url(&blob_url);
            return;
        };
        let Some(document) = window.document() else {
            let _ = web_sys::Url::revoke_object_url(&blob_url);
            return;
        };
        let Ok(anchor_element) = document.create_element("a") else {
            let _ = web_sys::Url::revoke_object_url(&blob_url);
            return;
        };
        let Ok(anchor) = anchor_element.dyn_into::<web_sys::HtmlAnchorElement>() else {
            let _ = web_sys::Url::revoke_object_url(&blob_url);
            return;
        };
        anchor.set_href(&blob_url);
        anchor.set_download(filename);
        anchor.click();
        let _ = web_sys::Url::revoke_object_url(&blob_url);
    }
}
