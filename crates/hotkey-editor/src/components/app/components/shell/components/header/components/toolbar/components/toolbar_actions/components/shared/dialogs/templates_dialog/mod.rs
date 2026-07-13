pub mod components;
mod data;
mod presentation;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::templates_dialog_body::TemplatesDialogBodyView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use presentation::{TemplatesDialogModel, use_templates_dialog};
use tw_macro::assert_component;

/// Connects the layout-templates browser to app state from the always-mounted toolbar, so it
/// opens from either the inline templates button or the burger drawer. It mounts the reusable
/// `WarcraftDialog` only while the shared open signal is set — the signal is the switch that
/// puts the dialog in the DOM — handing it the isolated gallery body region.
#[component]
pub fn TemplatesDialog() -> Element {
    let TemplatesDialogModel {
        open,
        on_open_change,
        cards,
    } = use_templates_dialog();
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
