pub mod components;
mod hooks;
mod style;

use crate::assert_component;
use components::export_button::ExportButton;
use dioxus::prelude::*;
use hooks::use_export_button;
use style::CLASS;

assert_component!(ExportButtonHost);

/// Connected wrapper and container: reads the live document from context to decide
/// whether the export button shows and to serialize on download, then renders the
/// presentational leaf inside its own container. The container owns how much space
/// the leaf gets, so the leaf sizes itself responsively with container-query units.
#[component]
pub fn ExportButtonHost() -> Element {
    let button = use_export_button();
    rsx! {
        div {
            class: CLASS,
            ExportButton { ..button }
        }
    }
}
