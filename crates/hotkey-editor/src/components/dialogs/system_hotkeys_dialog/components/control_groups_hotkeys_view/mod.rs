pub mod components;
mod props;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::dialogs::system_hotkeys_dialog::components::system_hotkeys_section::SystemHotkeysSection;
use components::control_groups_row::ControlGroupsRow;

pub use props::ControlGroupsHotkeysViewProps;

assert_component!(ControlGroupsHotkeysView);

/// The control-groups (1–10) hotkey editor: a ten-cell strip of editable slots.
#[component]
pub fn ControlGroupsHotkeysView(props: ControlGroupsHotkeysViewProps) -> Element {
    let loaded_keys = props.loaded_keys;
    let editing_section = props.editing_section;
    rsx! {
        SystemHotkeysSection {
            intro: "Hotkeys for control groups 1–10.",
            ControlGroupsRow { loaded_keys, editing_section }
        }
    }
}
