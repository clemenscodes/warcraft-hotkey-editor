pub mod components;
mod props;
mod style;

use components::system_key_picker_key::SystemKeyPickerKey;
use dioxus::prelude::*;
pub use props::SystemKeyPickerRowProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SystemKeyPickerRow);

/// One horizontal row of system-board keys.
#[component]
pub fn SystemKeyPickerRow(props: SystemKeyPickerRowProps) -> Element {
    let keys = props.keys;
    rsx! {
        div { class: CLASS,
            for key in keys {
                SystemKeyPickerKey { ..key }
            }
        }
    }
}
