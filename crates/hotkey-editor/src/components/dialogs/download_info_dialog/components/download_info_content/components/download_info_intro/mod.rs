mod style;

use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(DownloadInfoIntro);

/// The download dialog's lead-in line.
#[component]
pub fn DownloadInfoIntro() -> Element {
    rsx! {
        p { class: CLASS,
            "Place the file in your Documents folder, inside Warcraft III, then CustomKeyBindings. The filename must be exactly:"
        }
    }
}
