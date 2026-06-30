mod style;

use dioxus::prelude::*;

use style::DOWNLOAD_INFO_INTRO_STYLES;

/// The download dialog's lead-in line. Owns `.download-info-intro`.
#[component]
pub fn DownloadInfoIntro() -> Element {
    rsx! {
        document::Stylesheet { href: DOWNLOAD_INFO_INTRO_STYLES }
        p {
            class: "download-info-intro",
            "Place the file in your Documents folder, inside Warcraft III, then CustomKeyBindings. The filename must be exactly:"
        }
    }
}
