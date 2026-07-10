mod props;
mod style;

use crate::components::app::components::shell::components::shared::key_picker_board::KeyPickerBoardProps;
use crate::components::app::components::shell::components::shared::key_picker_board_host::KeyPickerBoardHost;
use dioxus::prelude::*;
pub use props::KeyPickerBodyProps;
use style::CLASS;
use tw_macro::assert_component;

/// The key picker's scrolling content region between the header and the panel
/// edge, holding the shared on-screen key picker board.
#[component]
pub fn KeyPickerBody(props: KeyPickerBodyProps) -> Element {
    let board = KeyPickerBoardProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            KeyPickerBoardHost { ..board }
        }
    }
}

assert_component!(KeyPickerBody);
