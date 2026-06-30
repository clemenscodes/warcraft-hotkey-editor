mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

assert_component!(DownloadInfoFilename);

/// The required filename shown as a boxed monospace chip.
#[component]
pub fn DownloadInfoFilename() -> Element {
    rsx! {
        div {
            class: CLASS,
            "CustomKeys.txt"
        }
    }
}
