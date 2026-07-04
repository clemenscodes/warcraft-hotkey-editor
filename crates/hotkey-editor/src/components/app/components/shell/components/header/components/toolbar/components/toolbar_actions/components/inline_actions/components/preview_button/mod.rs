mod hooks;
mod style;

use crate::assert_component;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::preview_dialog_host::PreviewDialogHost;
use dioxus::prelude::*;
use hooks::use_preview_button;
use style::CLASS;

assert_component!(PreviewButton);

/// Toolbar button that toggles the export preview pane, carrying the preview dialog it
/// opens. The button flips the shared open signal and the co-located host renders the
/// dialog — the desktop trigger; the burger renders its own copy for the compact
/// layout, where this button is hidden.
#[component]
pub fn PreviewButton() -> Element {
    let button = use_preview_button();
    rsx! {
        div {
            class: CLASS,
            ToolbarButton { ..button }
        }
        PreviewDialogHost {}
    }
}
