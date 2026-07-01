pub mod components;
mod style;

use crate::assert_component;
use components::download_info_filename::DownloadInfoFilename;
use components::download_info_intro::DownloadInfoIntro;
use components::download_info_warning::DownloadInfoWarning;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(DownloadInfoContent);

/// The download dialog's centered instruction block: the intro, the filename
/// chip, and the warning.
#[component]
pub fn DownloadInfoContent() -> Element {
    rsx! {
        div { class: CLASS,
            DownloadInfoIntro {}
            DownloadInfoFilename {}
            DownloadInfoWarning {}
        }
    }
}
