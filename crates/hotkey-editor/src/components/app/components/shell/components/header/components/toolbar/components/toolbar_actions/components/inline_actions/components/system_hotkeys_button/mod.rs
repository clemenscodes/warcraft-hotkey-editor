mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::SystemHotkeysDialog;
use dioxus::prelude::*;
use presentation::{use_system_hotkeys_button, SystemHotkeysButtonModel};
use style::CLASS;
use tw_macro::assert_component;

/// Toolbar button that opens the general (system) hotkeys dialog, carrying the dialog
/// it opens. The button flips the shared open signal and the co-located host renders
/// the dialog for the desktop trigger; the burger renders its own copy for the compact
/// layout.
#[component]
pub fn SystemHotkeysButton() -> Element {
    let SystemHotkeysButtonModel {
        icon,
        aria_label,
        aria_haspopup,
        aria_expanded,
        onclick,
    } = use_system_hotkeys_button();
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
        SystemHotkeysDialog {}
    }
}

assert_component!(SystemHotkeysButton);
