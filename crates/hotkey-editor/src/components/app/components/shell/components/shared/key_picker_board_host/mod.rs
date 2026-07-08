mod hooks;

use crate::components::app::components::shell::components::shared::key_picker_board::{
    KeyPickerBoard, KeyPickerBoardProps,
};
use dioxus::prelude::*;
use hooks::use_board_keyboard;
use tw_macro::assert_component;
assert_component!(KeyPickerBoardHost);

/// The interactive key picker: a [`KeyPickerBoard`] plus the keyboard listener and
/// focus that make it a picker. None of that is a dialog concern — drop this wherever a
/// hotkey is chosen (a dialog, a page, the gallery) and it focuses itself and resolves a
/// physical keypress into a pick with no wiring from its container. It forwards the
/// board's own props unchanged; the board it renders stays purely presentational, and
/// this host owns the side effects around it.
#[component]
pub fn KeyPickerBoardHost(props: KeyPickerBoardProps) -> Element {
    use_board_keyboard(&props);
    rsx! {
        KeyPickerBoard { ..props }
    }
}
