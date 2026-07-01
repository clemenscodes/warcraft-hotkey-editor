pub mod components;
mod hooks;
mod props;

use dioxus::prelude::*;

use super::dialog::Dialog;
use crate::assert_component;
use components::key_picker_board::KeyPickerBoard;
use hooks::use_key_picker;

pub use props::{KeyPickerCell, KeyPickerCellState, KeyPickerProps};

assert_component!(KeyPicker);

/// Assigns a hotkey from an on-screen keyboard. A variant of the `Dialog` base:
/// the hook shapes the open signal and the board, and the body composes the shell
/// with the board of keys.
#[component]
pub fn KeyPicker(props: KeyPickerProps) -> Element {
    let model = use_key_picker(&props);
    let title = props.title.clone();
    rsx! {
        Dialog {
            open: model.open,
            title,
            KeyPickerBoard { ..model.board }
        }
    }
}
