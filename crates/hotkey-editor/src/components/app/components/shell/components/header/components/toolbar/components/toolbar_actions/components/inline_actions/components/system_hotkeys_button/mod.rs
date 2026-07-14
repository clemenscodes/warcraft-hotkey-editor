mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::SystemHotkeysDialog;
use dioxus::prelude::*;
use presentation::{SystemHotkeysButtonPresentation, use_system_hotkeys_button};
use style::CLASS;
use tw_macro::assert_component;

/// The inline system-hotkeys action: the toolbar cog button that opens the system-hotkeys
/// editor, and that editor dialog, mounted beneath it. It owns the editor's open signal locally,
/// so the dialog is part of the button and travels with it.
#[component]
pub fn SystemHotkeysButton() -> Element {
    let SystemHotkeysButtonPresentation {
        icon,
        aria_label,
        aria_haspopup,
        aria_expanded,
        open,
        onclick,
        on_open_change,
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
            SystemHotkeysDialog {
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(SystemHotkeysButton);
