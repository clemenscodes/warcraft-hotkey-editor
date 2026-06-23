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
        // Clearing the value before opening the picker ensures the change event
        // fires even when the user re-selects the same file. A file input only
        // dispatches `change` when the chosen file differs from its current
        // value, so without this reset a repeat import of the same path is a
        // silent no-op.
        input_element.set_value("");
        input_element.click();
    }
}
