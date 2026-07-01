use crate::components::shared::icons::ICON_PREVIEW;
use crate::components::shared::toolbar_button::ToolbarButton;
use crate::services::overlay_state::OverlayState;
use dioxus::prelude::*;

/// Toolbar button that toggles the export preview pane.
#[component]
pub fn PreviewButton() -> Element {
    let overlay = use_context::<OverlayState>();
    let mut preview_open = overlay.preview_open;
    let preview_visible = *preview_open.read();
    let preview_label = if preview_visible {
        "Hide preview"
    } else {
        "Preview"
    };
    let toggle_preview = move |_| {
        let next_value = !*preview_open.read();
        preview_open.set(next_value);
    };
    rsx! {
        ToolbarButton {
            icon: ICON_PREVIEW,
            aria_label: preview_label,
            aria_pressed: preview_visible,
            onclick: toggle_preview,
        }
    }
}
