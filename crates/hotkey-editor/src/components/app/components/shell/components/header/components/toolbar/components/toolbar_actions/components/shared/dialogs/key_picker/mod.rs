mod hooks;
mod logic;
mod props;

use super::dialog::{Dialog, DialogProps};
use dioxus::prelude::*;
use hooks::use_key_picker;
pub use props::{KeyPickerCell, KeyPickerCellState, KeyPickerProps};

/// Assigns an ability hotkey from an on-screen letter keyboard. A variant of the
/// `Dialog` base: the hook shapes the open signal and the shared board, and the body
/// composes the dialog shell around the [`KeyPickerBoardHost`]. The picker owns no
/// keyboard listening or focus of its own — the board host does.
use tw_macro::assert_component;
assert_component!(KeyPicker);
#[component]
pub fn KeyPicker(props: KeyPickerProps) -> Element {
    let model = use_key_picker(&props);
    rsx! {
        Dialog { ..DialogProps::from(&model) }
    }
}
