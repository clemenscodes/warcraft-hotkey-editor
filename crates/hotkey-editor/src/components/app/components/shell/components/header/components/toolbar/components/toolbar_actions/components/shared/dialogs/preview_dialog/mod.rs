pub mod components;
mod data;
mod presentation;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::preview_textarea_host::PreviewTextareaHostView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use presentation::PreviewDialogPresentation;
use presentation::use_preview_dialog;
use tw_macro::assert_component;

/// The export preview dialog: a read-only pane showing the serialized CustomKeys.txt.
/// It reads the shared preview open signal from the overlay context and mounts the
/// reusable `WarcraftDialog` with the isolated preview content as its body region only
/// while open; the headless dialog derives its own close from `on_open_change`. The
/// toolbar preview button and the burger drawer both flip the shared signal.
#[component]
pub fn PreviewDialog() -> Element {
    let PreviewDialogPresentation {
        open,
        on_open_change,
    } = use_preview_dialog();
    let body = PreviewTextareaHostView;
    rsx! {
        if open {
            WarcraftDialog::<PreviewTextareaHostView, Empty> {
                title: data::TITLE,
                body,
                open: true,
                on_open_change,
            }
        }
    }
}

assert_component!(PreviewDialog);
