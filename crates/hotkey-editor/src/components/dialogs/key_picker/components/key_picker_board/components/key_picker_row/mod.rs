pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::key_picker_key::KeyPickerKey;
use style::CLASS;

pub use props::KeyPickerRowProps;

assert_component!(KeyPickerRow);

/// One horizontal row of picker keys.
#[component]
pub fn KeyPickerRow(props: KeyPickerRowProps) -> Element {
    let keys = props.keys;
    rsx! {
        div {
            class: CLASS,
            for key in keys {
                KeyPickerKey { ..key }
            }
        }
    }
}
