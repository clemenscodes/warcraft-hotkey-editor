pub mod components;
mod data;
mod hooks;
mod logic;
mod props;
mod view;

pub use view::SystemKeyPickerDialogView;
mod state;
mod style;

use components::system_key_picker_dialog_panel::SystemKeyPickerDialogPanel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogRoot;
use hooks::use_system_key_picker;
use logic::SystemKeyPickerDialogShell;
use props::SystemKeyPickerDialogProps;
use style::CLASS;
use tw_macro::assert_component;

/// Assigns a system or menu hotkey from a full on-screen keyboard (including keys the
/// letter picker does not offer, like function keys and the numpad). It owns its own
/// dialog shell: the composed hook shapes the title and board, the shell struct names
/// the open flag and panel, and this places the panel inside its own backdrop `div`
/// (the dimmed, centring layer) within the library `DialogRoot`. No project class
/// touches the library element. Focus and the keyboard listener belong to the picker,
/// not the dialog, so this component wires neither.
#[component]
pub fn SystemKeyPickerDialog(props: SystemKeyPickerDialogProps) -> Element {
    let model = use_system_key_picker(&props);
    use_body_scroll_lock(model.open);
    let SystemKeyPickerDialogShell {
        open,
        on_open_change,
        title,
        on_close,
        columns,
        on_pick,
        board_on_close,
    } = SystemKeyPickerDialogShell::from(&model);
    rsx! {
        DialogRoot {
            open,
            on_open_change,
            div {
                class: CLASS,
                SystemKeyPickerDialogPanel {
                    title,
                    on_close,
                    columns,
                    on_pick,
                    board_on_close,
                }
            }
        }
    }
}

assert_component!(SystemKeyPickerDialog);
