mod model;
mod view;

pub use view::TileOverrideKeyPickerView;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::KeyPicker;
use tw_macro::assert_component;

use model::TileOverrideKeyPickerModel;

/// Mounts the hotkey picker only while a hotkey field is being edited.
#[component]
pub fn TileOverrideKeyPicker(props: TileOverrideKeyPickerModel) -> Element {
    if !props.visible {
        return rsx! {};
    }
    let TileOverrideKeyPickerModel {
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
