pub mod components;
mod model;
mod presentation;
mod state;
mod style;
mod view;

pub use state::{KeyPickerCell, KeyPickerCellState};
pub use view::KeyPickerView;

use components::key_picker_panel::KeyPickerPanel;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogRoot;
use model::KeyPickerModel;
use presentation::{KeyPickerPresentation, use_key_picker_presentation};
use style::CLASS;
use tw_macro::assert_component;

/// Assigns an ability hotkey from an on-screen letter keyboard. It owns its own dialog
/// shell: it takes the `KeyPickerModel` props, the presentation builder mirrors the open
/// flag into a local signal and shapes the board, and this places the panel inside its
/// own backdrop `div` (the dimmed, centring layer) within the library `DialogRoot`. No
/// project class touches the library element.
#[component]
pub fn KeyPicker(props: KeyPickerModel) -> Element {
    let KeyPickerPresentation {
        open,
        title,
        columns,
        on_pick,
        on_close,
        on_open_change,
    } = use_key_picker_presentation(&props);
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
                    on_board_close: on_close,
                }
            }
        }
    }
}

assert_component!(KeyPicker);
