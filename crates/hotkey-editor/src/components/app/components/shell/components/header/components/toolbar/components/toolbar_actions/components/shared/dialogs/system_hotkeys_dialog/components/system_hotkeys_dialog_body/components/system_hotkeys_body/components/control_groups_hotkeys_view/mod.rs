pub mod components;
mod presentation;
mod style;

use components::control_groups_row::ControlGroupsRow;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_hotkeys_section_intro::SystemHotkeysSectionIntro;
use dioxus::prelude::*;
use presentation::{use_control_groups_hotkeys_view, ControlGroupsHotkeysViewModel};
use style::CLASS;
use tw_macro::assert_component;

/// The control-groups (1–10) hotkey editor: the domain-supplied intro caption above a
/// ten-cell strip of editable slots. Its slots read the editing section from the dialog
/// state context, so it threads nothing.
#[component]
pub fn ControlGroupsHotkeysView() -> Element {
    let ControlGroupsHotkeysViewModel { caption } = use_control_groups_hotkeys_view();
    rsx! {
        div {
            class: CLASS,
            SystemHotkeysSectionIntro { text: caption }
            ControlGroupsRow {}
        }
    }
}

assert_component!(ControlGroupsHotkeysView);
