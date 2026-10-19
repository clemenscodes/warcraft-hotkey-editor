pub(crate) const UPLOAD_INPUT_ELEMENT_ID: &str = "upload-customkeys-input";

pub(crate) fn trigger() {
    browser_kit::dom::UploadPicker::trigger(UPLOAD_INPUT_ELEMENT_ID);
}
