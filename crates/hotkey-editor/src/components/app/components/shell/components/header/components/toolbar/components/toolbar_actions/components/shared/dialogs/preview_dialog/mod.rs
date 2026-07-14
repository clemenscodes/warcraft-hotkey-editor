pub mod components;
mod data;
mod model;
mod view;

pub use view::PreviewDialogView;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::preview_textarea_host::PreviewTextareaHostView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use model::PreviewDialogModel;
use tw_macro::assert_component;

/// The export preview dialog: a read-only pane showing the serialized CustomKeys.txt, opened
/// by the trigger that owns its open signal (the inline preview button or the burger drawer,
/// each with its own instance). It mounts the reusable `WarcraftDialog` with the isolated
/// preview content as its body region only while `open`; the headless dialog derives its own
/// close from `on_open_change`.
#[component]
pub fn PreviewDialog(props: PreviewDialogModel) -> Element {
    let open = props.open;
    let on_open_change = props.on_open_change;
    let body = PreviewTextareaHostView;
    rsx! {
        if open {
            WarcraftDialog::<PreviewTextareaHostView,Empty> {
                title: data::TITLE,
                body,
                open: true,
                on_open_change,
            }
        }
    }
}

assert_component!(PreviewDialog);
