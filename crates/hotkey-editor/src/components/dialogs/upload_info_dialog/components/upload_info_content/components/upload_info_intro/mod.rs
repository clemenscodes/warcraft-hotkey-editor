mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

assert_component!(UploadInfoIntro);

/// The import dialog's lead-in line.
#[component]
pub fn UploadInfoIntro() -> Element {
    rsx! {
        p {
            class: CLASS,
            "Open your Documents folder, navigate to Warcraft III, then CustomKeyBindings, and select this file:"
        }
    }
}
