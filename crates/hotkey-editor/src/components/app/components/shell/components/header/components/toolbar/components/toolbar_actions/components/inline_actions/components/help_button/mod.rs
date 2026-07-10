mod hooks;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::HelpDialogHost;
use dioxus::prelude::*;
use hooks::use_help_button;
use style::CLASS;
use tw_macro::assert_component;

/// Toolbar button that opens the onboarding help dialog, carrying the dialog it opens.
/// The button flips the shared open signal and the co-located host renders the dialog
/// for the desktop trigger; the burger renders its own copy for the compact layout.
#[component]
pub fn HelpButton() -> Element {
    let button = use_help_button();
    rsx! {
        div {
            class: CLASS,
            ToolbarButton { ..button }
        }
        HelpDialogHost {}
    }
}

assert_component!(HelpButton);
