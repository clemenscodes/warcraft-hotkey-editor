pub mod components;
mod props;
mod style;

use crate::assert_component;
use crate::components::dialogs::system_hotkeys_dialog::components::key_capture_cell::{
    KeyCaptureCell, KeyCaptureCellProps,
};
use components::system_hotkeys_list_entry_label::{
    SystemHotkeysListEntryLabel, SystemHotkeysListEntryLabelProps,
};
use dioxus::prelude::*;
pub use props::SystemHotkeysListEntryProps;
use style::CLASS;
assert_component!(SystemHotkeysListEntry);

/// One hotkey row: the binding name beside its editable key chip.
#[component]
pub fn SystemHotkeysListEntry(props: SystemHotkeysListEntryProps) -> Element {
    let label = SystemHotkeysListEntryLabelProps::from(&props);
    let cell = KeyCaptureCellProps::from(&props);
    rsx! {
        li {
            class: CLASS,
            SystemHotkeysListEntryLabel { ..label }
            KeyCaptureCell { ..cell }
        }
    }
}
