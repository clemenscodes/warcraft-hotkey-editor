pub mod components;
mod style;

use crate::assert_component;
use components::upload_info_filename::UploadInfoFilename;
use components::upload_info_intro::UploadInfoIntro;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(UploadInfoContent);

/// The import dialog's centered instruction block: the intro and the filename
/// chip.
#[component]
pub fn UploadInfoContent() -> Element {
    rsx! {
        div { class: CLASS,
            UploadInfoIntro {}
            UploadInfoFilename {}
        }
    }
}
