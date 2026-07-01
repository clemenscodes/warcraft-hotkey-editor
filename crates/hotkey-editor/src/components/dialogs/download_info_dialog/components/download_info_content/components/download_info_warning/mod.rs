mod style;

use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(DownloadInfoWarning);

/// The amber callout warning that the filename and saved positions are fixed.
#[component]
pub fn DownloadInfoWarning() -> Element {
    rsx! {
        p { class: CLASS,
            "Any other filename will not be detected by Warcraft III. Note: button positions in saved custom games are fixed at save time and will not update, even if hotkeys change."
        }
    }
}
