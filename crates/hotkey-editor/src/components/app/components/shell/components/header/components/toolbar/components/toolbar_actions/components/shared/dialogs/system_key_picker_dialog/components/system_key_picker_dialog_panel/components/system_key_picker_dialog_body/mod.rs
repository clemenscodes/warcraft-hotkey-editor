mod props;
mod style;

use crate::components::app::components::shell::components::shared::key_picker_board::KeyPickerBoardProps;
use crate::components::app::components::shell::components::shared::key_picker_board_host::KeyPickerBoardHost;
use dioxus::prelude::*;
pub use props::SystemKeyPickerDialogBodyProps;
use style::CLASS;
use tw_macro::assert_component;

/// The system key picker's scrolling content region between the header and the
/// panel edge, holding the shared full-bleed on-screen keyboard board.
#[component]
pub fn SystemKeyPickerDialogBody(props: SystemKeyPickerDialogBodyProps) -> Element {
    let board = KeyPickerBoardProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            KeyPickerBoardHost { ..board }
        }
    }
}

assert_component!(SystemKeyPickerDialogBody);
