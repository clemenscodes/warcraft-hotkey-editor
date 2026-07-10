mod props;
mod view;

pub use view::TileOverrideKeyPickerView;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::KeyPicker;
use tw_macro::assert_component;

use props::TileOverrideKeyPickerProps;

/// Mounts the hotkey picker only while a hotkey field is being edited.
#[component]
pub fn TileOverrideKeyPicker(props: TileOverrideKeyPickerProps) -> Element {
    if !props.visible {
        return rsx! {};
    }
    let TileOverrideKeyPickerProps {
        title,
        rows,
        on_pick,
        on_close,
        ..
    } = props;
    rsx! {
        KeyPicker {
            title,
            rows,
            open: true,
            allow_conflict_pick: false,
            on_pick,
            on_close,
        }
    }
}

assert_component!(TileOverrideKeyPicker);
