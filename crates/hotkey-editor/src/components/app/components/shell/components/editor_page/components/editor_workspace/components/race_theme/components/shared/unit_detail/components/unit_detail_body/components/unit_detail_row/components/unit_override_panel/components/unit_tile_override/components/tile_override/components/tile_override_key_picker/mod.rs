mod props;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::{KeyPicker, KeyPickerProps};
use tw_macro::assert_component;

pub use props::TileOverrideKeyPickerProps;

/// Mounts the hotkey picker only while a hotkey field is being edited.
#[component]
pub fn TileOverrideKeyPicker(props: TileOverrideKeyPickerProps) -> Element {
    if !props.visible {
        return rsx! {};
    }
    rsx! {
        KeyPicker { ..KeyPickerProps::from(&props) }
    }
}

assert_component!(TileOverrideKeyPicker);
