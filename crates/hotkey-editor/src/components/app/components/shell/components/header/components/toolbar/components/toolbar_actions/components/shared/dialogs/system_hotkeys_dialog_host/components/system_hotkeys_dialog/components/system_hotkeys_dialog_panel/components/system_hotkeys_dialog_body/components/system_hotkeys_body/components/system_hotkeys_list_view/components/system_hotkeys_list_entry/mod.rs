pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::components::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::system_hotkeys_list_view::components::system_hotkeys_list_entry::components::key_capture::{
    KeyCapture, KeyCaptureProps,
};
use components::system_hotkeys_list_entry_label::{
    SystemHotkeysListEntryLabel, SystemHotkeysListEntryLabelProps,
};
use dioxus::prelude::*;
pub use props::SystemHotkeysListEntryProps;
use style::CLASS;
use tw_macro::assert_component;

/// One hotkey row: the binding name beside its editable key chip.
#[component]
pub fn SystemHotkeysListEntry(props: SystemHotkeysListEntryProps) -> Element {
    let label = SystemHotkeysListEntryLabelProps::from(&props);
    let cell = KeyCaptureProps::from(&props);
    rsx! {
        li {
            class: CLASS,
            SystemHotkeysListEntryLabel { ..label }
            KeyCapture { ..cell }
        }
    }
}

assert_component!(SystemHotkeysListEntry);
