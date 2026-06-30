pub mod components;
mod style;

use dioxus::prelude::*;

use components::download_info_filename::DownloadInfoFilename;
use components::download_info_intro::DownloadInfoIntro;
use components::download_info_warning::DownloadInfoWarning;
use style::DOWNLOAD_INFO_CONTENT_STYLES;

/// The download dialog's centered instruction block. Owns `.download-info-content`
/// and stacks the intro, the filename chip, and the warning.
#[component]
pub fn DownloadInfoContent() -> Element {
    rsx! {
        document::Stylesheet { href: DOWNLOAD_INFO_CONTENT_STYLES }
        div {
            class: "download-info-content",
            DownloadInfoIntro {}
            DownloadInfoFilename {}
            DownloadInfoWarning {}
        }
    }
}
