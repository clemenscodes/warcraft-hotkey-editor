mod props;

use dioxus::prelude::*;

use crate::components::dialogs::key_picker::KeyPicker;

pub use props::TileOverrideKeyPickerProps;

/// Mounts the hotkey picker only while a hotkey field is being edited.
#[component]
pub fn TileOverrideKeyPicker(props: TileOverrideKeyPickerProps) -> Element {
    if !props.visible {
        return rsx! {};
    }
    rsx! {
        KeyPicker {
            title: props.title,
            rows: props.rows,
            open: true,
            on_pick: props.on_pick,
            on_close: props.on_close,
        }
    }
}
