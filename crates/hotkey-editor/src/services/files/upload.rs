use wasm_bindgen::JsCast;

pub(crate) const UPLOAD_INPUT_ELEMENT_ID: &str = "upload-customkeys-input";

pub(crate) struct UploadPicker;

impl UploadPicker {
    pub(crate) fn trigger() {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };
        let Some(element) = document.get_element_by_id(UPLOAD_INPUT_ELEMENT_ID) else {
            return;
        };
        let Ok(input_element) = element.dyn_into::<web_sys::HtmlInputElement>() else {
            return;
        };
        input_element.click();
    }
}
