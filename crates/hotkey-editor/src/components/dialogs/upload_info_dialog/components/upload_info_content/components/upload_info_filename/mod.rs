mod style;

use dioxus::prelude::*;

use style::UPLOAD_INFO_FILENAME_STYLES;

/// The filename to select, shown as a boxed monospace chip. Owns
/// `.upload-info-filename`.
#[component]
pub fn UploadInfoFilename() -> Element {
    rsx! {
        document::Stylesheet { href: UPLOAD_INFO_FILENAME_STYLES }
        div {
            class: "upload-info-filename",
            "CustomKeys.txt"
        }
    }
}
