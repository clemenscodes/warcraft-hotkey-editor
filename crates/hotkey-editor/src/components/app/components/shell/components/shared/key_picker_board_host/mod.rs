mod hooks;
mod props;
mod view;

pub use view::KeyPickerBoardHostView;

use crate::components::app::components::shell::components::shared::key_picker_board::KeyPickerBoard;
use dioxus::prelude::*;
use hooks::use_board_keyboard;
use props::KeyPickerBoardHostProps;
use tw_macro::assert_component;

/// The interactive key picker: a [`KeyPickerBoard`] plus the keyboard listener and
/// focus that make it a picker. None of that is a dialog concern — drop this wherever a
/// hotkey is chosen (a dialog, a page, the gallery) and it focuses itself and resolves a
/// physical keypress into a pick with no wiring from its container. It threads the
/// board's columns and handlers down by name; the board it renders stays purely
/// presentational, and this host owns the side effects around it.
#[component]
pub fn KeyPickerBoardHost(props: KeyPickerBoardHostProps) -> Element {
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
