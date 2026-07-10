mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::info_dialog::data::FILENAME;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The required filename shown as a boxed monospace chip.
#[component]
pub fn InfoFilename() -> Element {
    rsx! {
        div { class: CLASS, "{FILENAME}" }
    }
}

assert_component!(InfoFilename);
