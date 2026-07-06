/// Opens the OS file picker of an existing `<input type="file">`, found by id.
/// Clears its value first so re-picking the same file still fires `change`.
pub struct UploadPicker;

impl UploadPicker {
    pub fn trigger(element_id: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            let document = web_sys::window().and_then(|window| window.document());
            let input = document
                .and_then(|document| document.get_element_by_id(element_id))
                .and_then(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok());
            if let Some(input_element) = input {
                input_element.set_value("");
                input_element.click();
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = element_id;
        }
    }
}
