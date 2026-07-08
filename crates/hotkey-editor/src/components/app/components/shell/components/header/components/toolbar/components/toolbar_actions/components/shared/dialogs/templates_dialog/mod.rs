pub mod components;
mod hooks;
mod logic;
mod props;
mod style;

use components::templates_dialog_body::TemplatesDialogBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use hooks::use_templates_dialog;
use logic::TemplatesDialogShell;
pub use props::TemplatesDialogProps;
use style::{CLASS, OVERLAY};
use tw_macro::assert_component;

assert_component!(TemplatesDialog);

/// Lets the player apply a bundled layout template. It owns its own dialog shell:
/// the hook resolves the template cards and apply handlers, the shell struct names
/// the header and scroll body, and this places them inside the backdrop and box.
#[component]
pub fn TemplatesDialog(props: TemplatesDialogProps) -> Element {
    use_body_scroll_lock(props.open);
    let view = use_templates_dialog(&props);
    let TemplatesDialogShell {
        open,
        on_open_change,
        header,
        body,
    } = TemplatesDialogShell::from(&view);
    if !open {
        return rsx! {};
    }
    rsx! {
        DialogRoot {
            class: OVERLAY,
            open,
            on_open_change,
            DialogContent {
                class: CLASS.to_library_class(),
                DialogHeader { ..header }
                TemplatesDialogBody { ..body }
            }
        }
    }
}
