mod hooks;
mod style;

use tw_macro::assert_component;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::templates_dialog_host::TemplatesDialogHost;
use dioxus::prelude::*;
use hooks::use_templates_button;
use style::CLASS;

assert_component!(TemplatesButton);

/// Toolbar button that opens the layout templates browser, carrying the dialog it
/// opens. The button flips the shared open signal and the co-located host renders the
/// dialog for the desktop trigger; the burger renders its own copy for the compact
/// layout.
#[component]
pub fn TemplatesButton() -> Element {
    let button = use_templates_button();
    rsx! {
        div {
            class: CLASS,
            ToolbarButton { ..button }
        }
        TemplatesDialogHost {}
    }
}
