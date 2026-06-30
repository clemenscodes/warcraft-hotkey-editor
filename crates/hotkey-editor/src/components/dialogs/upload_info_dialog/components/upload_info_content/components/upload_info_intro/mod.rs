mod style;

use dioxus::prelude::*;

use style::UPLOAD_INFO_INTRO_STYLES;

/// The import dialog's lead-in line. Owns `.upload-info-intro`.
#[component]
pub fn UploadInfoIntro() -> Element {
    rsx! {
        document::Stylesheet { href: UPLOAD_INFO_INTRO_STYLES }
        p {
            class: "upload-info-intro",
            "Open your Documents folder, navigate to Warcraft III, then CustomKeyBindings, and select this file:"
        }
    }
}
