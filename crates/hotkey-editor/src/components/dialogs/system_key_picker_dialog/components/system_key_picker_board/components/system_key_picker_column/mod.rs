pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::system_key_picker_row::SystemKeyPickerRow;
use style::CLASS;

pub use props::SystemKeyPickerColumnProps;

assert_component!(SystemKeyPickerColumn);

/// One column of the system keyboard board: a stack of key rows. The main keyboard
/// and the numpad are each rendered as one of these.
#[component]
pub fn SystemKeyPickerColumn(props: SystemKeyPickerColumnProps) -> Element {
    let rows = props.rows;
    rsx! {
        div {
            class: CLASS,
            for row in rows {
                SystemKeyPickerRow { ..row }
            }
        }
    }
}
