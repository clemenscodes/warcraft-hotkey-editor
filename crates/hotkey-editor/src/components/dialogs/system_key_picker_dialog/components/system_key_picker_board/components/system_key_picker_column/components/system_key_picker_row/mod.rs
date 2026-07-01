pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::system_key_picker_key::SystemKeyPickerKey;
use style::CLASS;

pub use props::SystemKeyPickerRowProps;

assert_component!(SystemKeyPickerRow);

/// One horizontal row of system-board keys.
#[component]
pub fn SystemKeyPickerRow(props: SystemKeyPickerRowProps) -> Element {
    let keys = props.keys;
    rsx! {
        div {
            class: CLASS,
            for key in keys {
                SystemKeyPickerKey { ..key }
            }
        }
    }
}
