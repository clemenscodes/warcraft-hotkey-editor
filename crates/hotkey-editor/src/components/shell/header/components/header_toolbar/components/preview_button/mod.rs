mod hooks;

use crate::components::shell::header::components::header_toolbar::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use hooks::use_preview_button;

/// Toolbar button that toggles the export preview pane.
#[component]
pub fn PreviewButton() -> Element {
    let button = use_preview_button();
    rsx! {
        ToolbarButton { ..button }
    }
}
