pub mod components;
mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::templates_dialog_body::TemplatesDialogBodyView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use presentation::{TemplatesDialogHostModel, use_templates_dialog_host};
use style::CLASS;
use tw_macro::assert_component;

/// Connects the layout-templates browser to app state and places it in the always-mounted
/// toolbar, so it opens from either the inline templates button or the burger drawer. It
/// renders the reusable `WarcraftDialog` directly, handing it the isolated gallery body
/// region; the headless dialog gates itself on the shared open signal.
#[component]
pub fn TemplatesDialogHost() -> Element {
    let TemplatesDialogHostModel {
        open,
        on_open_change,
        cards,
    } = use_templates_dialog_host();
    let body = TemplatesDialogBodyView { cards };
    rsx! {
        div {
            class: CLASS,
            WarcraftDialog::<TemplatesDialogBodyView, Empty> {
                title: data::TITLE,
                body,
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(TemplatesDialogHost);
