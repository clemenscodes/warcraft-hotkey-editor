pub mod components;
mod model;
mod presentation;
mod view;

pub use view::TemplatesDialogView;
mod style;

use components::templates_dialog_panel::TemplatesDialogPanel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogRoot;
use presentation::use_templates_dialog;
use presentation::TemplatesDialogShell;
use model::TemplatesDialogModel;
use style::CLASS;
use tw_macro::assert_component;

/// Lets the player apply a bundled layout template. It owns its own dialog shell:
/// the hook resolves the template cards and apply handlers, the shell struct shapes
/// the panel, and this places the panel inside its own backdrop `div` (the dimmed,
/// centring layer) within the library `DialogRoot`. No project class touches the
/// library element.
#[component]
pub fn TemplatesDialog(props: TemplatesDialogModel) -> Element {
    use_body_scroll_lock(props.open);
    let view = use_templates_dialog(&props);
    let TemplatesDialogShell {
        open,
        on_open_change,
        title,
        on_close,
        cards,
    } = TemplatesDialogShell::from(&view);
    if !open {
        return rsx! {};
    }
    rsx! {
        DialogRoot {
            open,
            on_open_change,
            div {
                class: CLASS,
                TemplatesDialogPanel { title, on_close, cards }
            }
        }
    }
}

assert_component!(TemplatesDialog);
