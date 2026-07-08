pub mod components;
mod hooks;
mod logic;
mod props;
mod style;

use components::key_picker_body::KeyPickerBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use hooks::use_key_picker;
use logic::KeyPickerShell;
pub use props::{KeyPickerCell, KeyPickerCellState, KeyPickerProps};
use style::{CLASS, OVERLAY};
use tw_macro::assert_component;

assert_component!(KeyPicker);

/// Assigns an ability hotkey from an on-screen letter keyboard. It owns its own
/// dialog shell: the hook mirrors the open flag and shapes the board, the shell
/// struct names the header and scroll body, and this places them inside the
/// backdrop and bordered box.
#[component]
pub fn KeyPicker(props: KeyPickerProps) -> Element {
    let model = use_key_picker(&props);
    use_body_scroll_lock(model.open);
    let KeyPickerShell {
        open,
        on_open_change,
        header,
        body,
    } = KeyPickerShell::from(&model);
    rsx! {
        DialogRoot {
            class: OVERLAY,
            open,
            on_open_change,
            DialogContent {
                class: CLASS.to_library_class(),
                DialogHeader { ..header }
                KeyPickerBody { ..body }
            }
        }
    }
}
