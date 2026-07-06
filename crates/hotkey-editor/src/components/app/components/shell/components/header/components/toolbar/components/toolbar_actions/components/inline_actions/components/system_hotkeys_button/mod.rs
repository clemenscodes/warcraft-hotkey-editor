mod hooks;
mod style;

use tw_macro::assert_component;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::SystemHotkeysDialogHost;
use dioxus::prelude::*;
use hooks::use_system_hotkeys_button;
use style::CLASS;

assert_component!(SystemHotkeysButton);

/// Toolbar button that opens the general (system) hotkeys dialog, carrying the dialog
/// it opens. The button flips the shared open signal and the co-located host renders
/// the dialog for the desktop trigger; the burger renders its own copy for the compact
/// layout.
#[component]
pub fn SystemHotkeysButton() -> Element {
    let button = use_system_hotkeys_button();
    rsx! {
        div {
            class: CLASS,
            ToolbarButton { ..button }
        }
        SystemHotkeysDialogHost {}
    }
}
