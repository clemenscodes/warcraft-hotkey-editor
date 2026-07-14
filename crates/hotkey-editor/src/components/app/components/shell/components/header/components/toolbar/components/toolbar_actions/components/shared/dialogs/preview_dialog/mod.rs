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
