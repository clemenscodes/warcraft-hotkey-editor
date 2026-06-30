mod style;

use dioxus::prelude::*;

use style::DOWNLOAD_INFO_FILENAME_STYLES;

/// The required filename shown as a boxed monospace chip. Owns
/// `.download-info-filename`.
#[component]
pub fn DownloadInfoFilename() -> Element {
    rsx! {
        document::Stylesheet { href: DOWNLOAD_INFO_FILENAME_STYLES }
        div {
            class: "download-info-filename",
            "CustomKeys.txt"
        }
    }
}
