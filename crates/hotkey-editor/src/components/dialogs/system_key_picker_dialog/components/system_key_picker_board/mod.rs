pub mod components;
mod hooks;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::system_key_picker_column::SystemKeyPickerColumn;
use hooks::use_board_focus;
use style::CLASS;

pub use props::SystemKeyPickerBoardProps;

assert_component!(SystemKeyPickerBoard);

/// The on-screen system keyboard: the main keyboard column beside the numpad
/// column. Owns the focus side effect; the keydown handler arrives shaped as a
/// prop.
#[component]
pub fn SystemKeyPickerBoard(props: SystemKeyPickerBoardProps) -> Element {
    use_board_focus();
    let columns = props.columns;
    let onkeydown = props.onkeydown;
    rsx! {
        div {
            class: CLASS,
            tabindex: "-1",
            onkeydown,
            for column in columns {
                SystemKeyPickerColumn { ..column }
            }
        }
    }
}
