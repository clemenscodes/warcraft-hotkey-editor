pub mod components;
mod logic;
mod props;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_hotkeys_section::{
    SystemHotkeysSection, SystemHotkeysSectionProps,
};
use dioxus::prelude::*;
pub use props::ControlGroupsHotkeysViewProps;

/// The control-groups (1–10) hotkey editor: a ten-cell strip of editable slots.
#[component]
pub fn ControlGroupsHotkeysView(props: ControlGroupsHotkeysViewProps) -> Element {
    rsx! {
        SystemHotkeysSection { ..SystemHotkeysSectionProps::from(&props) }
    }
}
