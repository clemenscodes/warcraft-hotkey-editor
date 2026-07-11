mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::templates_dialog_host::TemplatesDialogHost;
use dioxus::prelude::*;
use presentation::{use_templates_button, TemplatesButtonModel};
use style::CLASS;
use tw_macro::assert_component;

/// Toolbar button that opens the layout templates browser, carrying the dialog it
/// opens. The button flips the shared open signal and the co-located host renders the
/// dialog for the desktop trigger; the burger renders its own copy for the compact
/// layout.
#[component]
pub fn TemplatesButton() -> Element {
    let TemplatesButtonModel {
        icon,
        aria_label,
        aria_haspopup,
        aria_expanded,
        onclick,
    } = use_templates_button();
    rsx! {
        div {
            class: CLASS,
            ToolbarButton {
                icon,
                aria_label,
                aria_haspopup,
                aria_expanded,
                onclick,
            }
        }
        TemplatesDialogHost {}
    }
}

assert_component!(TemplatesButton);
