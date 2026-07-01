mod style;

use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(UploadInfoFilename);

/// The filename to select, shown as a boxed monospace chip.
#[component]
pub fn UploadInfoFilename() -> Element {
    rsx! {
        div { class: CLASS, "CustomKeys.txt" }
    }
}
