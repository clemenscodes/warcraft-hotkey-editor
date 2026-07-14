pub mod components;
mod data;
mod model;
mod presentation;
mod view;

pub use view::TemplatesDialogView;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::templates_dialog_body::TemplatesDialogBodyView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use model::TemplatesDialogModel;
use presentation::{TemplatesDialogPresentation, use_templates_dialog};
use tw_macro::assert_component;

/// The layout-templates browser, opened by the trigger that owns its open signal (the inline
/// templates button or the burger drawer, each with its own instance). It mounts the reusable
/// `WarcraftDialog` only while `open` is set — the trigger's signal is the switch that puts the
/// dialog in the DOM — handing it the isolated gallery body region; the headless dialog derives
/// its own close from `on_open_change`.
#[component]
pub fn TemplatesDialog(props: TemplatesDialogModel) -> Element {
    let TemplatesDialogPresentation {
        open,
        on_open_change,
        cards,
    } = use_templates_dialog(&props);
    let body = TemplatesDialogBodyView { cards };
    rsx! {
        if open {
            WarcraftDialog::<TemplatesDialogBodyView, Empty> {
                title: data::TITLE,
                body,
                open: true,
                on_open_change,
            }
        }
    }
}

assert_component!(TemplatesDialog);
