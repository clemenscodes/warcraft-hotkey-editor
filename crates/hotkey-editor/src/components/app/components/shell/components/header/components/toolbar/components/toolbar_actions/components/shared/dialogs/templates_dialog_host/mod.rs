pub mod components;
mod hooks;
mod style;

use components::templates_dialog::TemplatesDialog;
use dioxus::prelude::*;
use hooks::{TemplatesDialogHostModel, use_templates_dialog_host};
use style::CLASS;
use tw_macro::assert_component;

/// Connects the layout-templates browser to app state and places it in the
/// always-mounted toolbar, so it opens from either the inline templates button or the
/// burger drawer. The dialog self-gates on the shared open signal.
#[component]
pub fn TemplatesDialogHost() -> Element {
    let TemplatesDialogHostModel {
        loaded_keys,
        upload_status,
        open,
    } = use_templates_dialog_host();
    rsx! {
        div {
            class: CLASS,
            TemplatesDialog { loaded_keys, upload_status, open }
        }
    }
}

assert_component!(TemplatesDialogHost);
