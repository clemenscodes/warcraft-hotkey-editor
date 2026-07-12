mod model;
mod view;

pub use view::KeyPickerBodyView;
mod style;

use crate::components::app::components::shell::components::shared::key_picker_board_host::KeyPickerBoardHost;
use dioxus::prelude::*;
use model::KeyPickerBodyModel;
use style::CLASS;
use tw_macro::assert_component;

/// The key picker's scrolling content region between the header and the panel
/// edge, holding the shared on-screen key picker board.
#[component]
pub fn KeyPickerBody(props: KeyPickerBodyModel) -> Element {
    let columns = props.columns.clone();
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

assert_component!(KeyPickerBody);
