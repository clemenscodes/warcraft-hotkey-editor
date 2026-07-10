pub mod components;
mod props;
mod style;

use components::key_picker_key::KeyPickerKey;
use dioxus::prelude::*;
use props::KeyPickerRowProps;
use style::CLASS;
use tw_macro::assert_component;

/// One horizontal row of picker keys.
#[component]
pub fn KeyPickerRow(props: KeyPickerRowProps) -> Element {
    let keys = props.keys;
    let on_pick = props.on_pick;
    rsx! {
        div { class: CLASS,
            for cell in keys {
                KeyPickerKey { cell, on_pick }
            }
        }
    }
}

assert_component!(KeyPickerRow);
