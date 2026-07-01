use dioxus::prelude::*;

use crate::components::shared::icons::ICON_GRID;
use crate::services::overlay_state::OverlayState;

const GRID_LAYOUT_BUTTON_STYLES: Asset =
    asset!("/src/components/actions/grid_layout_button/grid_layout_button.css");

/// Prominent header call-to-action that opens the global grid-layout editor.
/// Deliberately styled apart from the icon-only toolbar buttons; its styling
/// lives in `grid_layout_button.css` under the `.grid-layout-button` class.
#[component]
pub fn GridLayoutButton() -> Element {
    let overlay = use_context::<OverlayState>();
    let mut layout_dialog_open = overlay.layout_dialog_open;
    let is_open = layout_dialog_open();
    let toggle_layout_dialog = move |_| {
        let next = !*layout_dialog_open.read();
        layout_dialog_open.set(next);
    };
    rsx! {
        document::Stylesheet { href: GRID_LAYOUT_BUTTON_STYLES }
        button {
            class: "grid-layout-button",
            r#type: "button",
            aria_label: "Edit global hotkey layout",
            aria_haspopup: "dialog",
            aria_expanded: is_open,
            onclick: toggle_layout_dialog,
            span {
                class: "grid-layout-button-icon",
                aria_hidden: "true",
                dangerous_inner_html: ICON_GRID,
            }
            span { class: "grid-layout-button-label", "GRID LAYOUT" }
        }
    }
}
