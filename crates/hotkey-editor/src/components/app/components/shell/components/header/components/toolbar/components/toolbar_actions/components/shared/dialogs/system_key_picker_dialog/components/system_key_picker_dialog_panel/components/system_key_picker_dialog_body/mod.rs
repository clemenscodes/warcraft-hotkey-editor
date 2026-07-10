mod props;
mod view;

pub use view::SystemKeyPickerDialogBodyView;
mod style;

use crate::components::app::components::shell::components::shared::key_picker_board_host::KeyPickerBoardHost;
use dioxus::prelude::*;
use props::SystemKeyPickerDialogBodyProps;
use style::CLASS;
use tw_macro::assert_component;

/// The system key picker's scrolling content region between the header and the
/// panel edge, holding the shared full-bleed on-screen keyboard board.
#[component]
pub fn SystemKeyPickerDialogBody(props: SystemKeyPickerDialogBodyProps) -> Element {
    let columns = props.columns;
    let on_pick = props.on_pick;
    let on_close = props.on_close;
    rsx! {
        div {
            class: CLASS,
            KeyPickerBoardHost {
                columns,
                on_pick,
                on_close,
            }
        }
    }
}

assert_component!(SystemKeyPickerDialogBody);
