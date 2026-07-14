pub mod components;
mod model;
mod view;

pub use view::KeyPickerRowView;
mod style;

use components::key_picker_key::KeyPickerKey;
use dioxus::prelude::*;
use model::KeyPickerRowModel;
use style::CLASS;
use tw_macro::assert_component;

/// One horizontal row of picker keys.
#[component]
pub fn KeyPickerRow(props: KeyPickerRowModel) -> Element {
    let keys = props.keys;
    let on_pick = props.on_pick;
    rsx! {
        div {
            class: CLASS,
            for cell in keys {
                KeyPickerKey {
                    cell,
                    on_pick,
                }
            }
        }
    }
}

assert_component!(KeyPickerRow);
