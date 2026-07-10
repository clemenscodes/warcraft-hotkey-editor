pub mod components;
mod hooks;
mod logic;
mod props;
mod state;
mod style;

use components::key_picker_panel::KeyPickerPanel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogRoot;
use hooks::use_key_picker;
use logic::KeyPickerShell;
use props::KeyPickerProps;
pub use state::{KeyPickerCell, KeyPickerCellState};
use style::CLASS;
use tw_macro::assert_component;

/// Assigns an ability hotkey from an on-screen letter keyboard. It owns its own
/// dialog shell: the hook mirrors the open flag and shapes the board, the shell struct
/// names the open flag and panel, and this places the panel inside its own backdrop
/// `div` (the dimmed, centring layer) within the library `DialogRoot`. No project class
/// touches the library element.
#[component]
pub fn KeyPicker(props: KeyPickerProps) -> Element {
    let model = use_key_picker(&props);
    use_body_scroll_lock(model.open);
    let KeyPickerShell {
        open,
        on_open_change,
        title,
        on_close,
        columns,
        on_pick,
        on_board_close,
    } = KeyPickerShell::from(&model);
    rsx! {
        DialogRoot {
            open,
            on_open_change,
            div {
                class: CLASS,
                KeyPickerPanel {
                    title,
                    on_close,
                    columns,
                    on_pick,
                    on_board_close,
                }
            }
        }
    }
}

assert_component!(KeyPicker);
