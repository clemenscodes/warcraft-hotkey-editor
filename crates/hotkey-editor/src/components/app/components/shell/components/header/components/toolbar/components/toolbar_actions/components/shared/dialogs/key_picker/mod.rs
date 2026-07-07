pub mod components;
mod hooks;
mod logic;
mod props;

use super::dialog::{Dialog, DialogProps};
use dioxus::prelude::*;
use hooks::use_key_picker;
pub use props::{KeyPickerCell, KeyPickerCellState, KeyPickerProps};

/// Assigns a hotkey from an on-screen keyboard. A variant of the `Dialog` base:
/// the hook shapes the open signal and the board, and the body composes the shell
/// with the board of keys.
use tw_macro::assert_component;
assert_component!(KeyPicker);
#[component]
pub fn KeyPicker(props: KeyPickerProps) -> Element {
    let model = use_key_picker(&props);
    rsx! {
        Dialog { ..DialogProps::from(&model) }
    }
}
