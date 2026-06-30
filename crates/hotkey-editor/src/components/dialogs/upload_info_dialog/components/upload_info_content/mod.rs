pub mod components;
mod style;

use dioxus::prelude::*;

use components::upload_info_filename::UploadInfoFilename;
use components::upload_info_intro::UploadInfoIntro;
use style::UPLOAD_INFO_CONTENT_STYLES;

/// The import dialog's centered instruction block. Owns `.upload-info-content`
/// and stacks the intro and the filename chip.
#[component]
pub fn UploadInfoContent() -> Element {
    rsx! {
        document::Stylesheet { href: UPLOAD_INFO_CONTENT_STYLES }
        div {
            class: "upload-info-content",
            UploadInfoIntro {}
            UploadInfoFilename {}
        }
    }
}
