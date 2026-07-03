mod style;

use crate::assert_component;
use crate::components::dialogs::info_dialogs::info_dialog::data::FILENAME;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(InfoFilename);

/// The required filename shown as a boxed monospace chip.
#[component]
pub fn InfoFilename() -> Element {
    rsx! {
        div { class: CLASS, "{FILENAME}" }
    }
}
