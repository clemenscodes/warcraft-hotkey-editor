pub mod components;
mod props;
mod style;

use components::key_picker_key::KeyPickerKey;
use dioxus::prelude::*;
pub use props::KeyPickerRowProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(KeyPickerRow);

/// One horizontal row of picker keys.
#[component]
pub fn KeyPickerRow(props: KeyPickerRowProps) -> Element {
    let keys = props.keys;
    rsx! {
        div { class: CLASS,
            for key in keys {
                KeyPickerKey { ..key }
            }
        }
    }
}
