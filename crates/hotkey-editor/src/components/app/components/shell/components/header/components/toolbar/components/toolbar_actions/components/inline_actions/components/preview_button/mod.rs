pub mod components;
mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::preview_textarea_host::PreviewTextareaHostView;
use dioxus::prelude::*;
use presentation::{use_preview_button, PreviewButtonModel};
use style::CLASS;
use tw_macro::assert_component;

/// Toolbar button that toggles the export preview, carrying the preview dialog it opens.
/// The button flips the shared open signal; the co-located `WarcraftDialog` — a sibling of
/// the (laptop-and-up) button slot, so it stays mounted even where that slot is hidden and
/// the burger flips the same signal — renders the isolated preview content as its body
/// region.
#[component]
pub fn PreviewButton() -> Element {
    let PreviewButtonModel {
        icon,
        aria_label,
        aria_pressed,
        onclick,
        open,
        on_open_change,
    } = use_preview_button();
    let body = PreviewTextareaHostView;
    rsx! {
        div {
            class: CLASS,
            ToolbarButton {
                icon,
                aria_label,
                aria_pressed,
                onclick,
            }
        }
        WarcraftDialog::<PreviewTextareaHostView> {
            title: data::TITLE,
            body,
            open,
            on_open_change,
        }
    }
}

assert_component!(PreviewButton);
