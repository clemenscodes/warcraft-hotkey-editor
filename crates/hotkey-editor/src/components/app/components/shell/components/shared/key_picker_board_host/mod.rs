mod model;
mod presentation;
mod view;

pub use view::KeyPickerBoardHostView;

use crate::components::app::components::shell::components::shared::key_picker_board::KeyPickerBoard;
use dioxus::prelude::*;
use model::KeyPickerBoardHostModel;
use presentation::use_board_keyboard;
use tw_macro::assert_component;

#[component]
pub fn KeyPickerBoardHost(props: KeyPickerBoardHostModel) -> Element {
    use_board_keyboard(&props);
    let columns = props.columns;
    let on_pick = props.on_pick;
    let on_close = props.on_close;
    rsx! {
        KeyPickerBoard {
            columns,
            on_pick,
            on_close,
        }
    }
}

assert_component!(KeyPickerBoardHost);
