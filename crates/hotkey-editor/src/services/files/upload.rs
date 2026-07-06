/// The app's upload seam: opens the OS file picker for the hidden CustomKeys
/// file input. The DOM mechanics live in `browser_kit::dom::UploadPicker`.
pub(crate) const UPLOAD_INPUT_ELEMENT_ID: &str = "upload-customkeys-input";

pub(crate) struct UploadPicker;

impl UploadPicker {
    pub(crate) fn trigger() {
        browser_kit::dom::UploadPicker::trigger(UPLOAD_INPUT_ELEMENT_ID);
    }
}
