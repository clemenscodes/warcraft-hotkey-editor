use dioxus::prelude::*;

use crate::components::shared::icons::ICON_PREVIEW;
use crate::components::shared::toolbar_button::ToolbarButton;

#[derive(Props, Clone, PartialEq)]
pub struct PreviewButtonProps {
    pub preview_open: Signal<bool>,
}

/// Toolbar button that toggles the export preview pane.
#[component]
pub fn PreviewButton(props: PreviewButtonProps) -> Element {
    let mut preview_open = props.preview_open;
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
