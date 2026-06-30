mod style;

use dioxus::prelude::*;

use style::DOWNLOAD_INFO_WARNING_STYLES;

/// The amber callout warning that the filename and saved positions are fixed.
/// Owns `.download-info-warning`.
#[component]
pub fn DownloadInfoWarning() -> Element {
    rsx! {
        document::Stylesheet { href: DOWNLOAD_INFO_WARNING_STYLES }
        p {
            class: "download-info-warning",
            "Any other filename will not be detected by Warcraft III. Note: button positions in saved custom games are fixed at save time and will not update, even if hotkeys change."
        }
    }
}
