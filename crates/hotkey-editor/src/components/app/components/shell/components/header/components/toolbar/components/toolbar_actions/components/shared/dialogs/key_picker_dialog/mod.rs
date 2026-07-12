pub mod components;
mod model;
mod presentation;
mod state;
mod view;

pub use state::{KeyPickerCell, KeyPickerCellState};
pub use view::KeyPickerDialogView;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::key_picker_body::KeyPickerBodyView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use model::KeyPickerDialogModel;
use presentation::{KeyPickerDialogPresentation, use_key_picker_dialog_presentation};
use tw_macro::assert_component;

/// Assigns an ability hotkey from an on-screen letter keyboard. It takes the
/// `KeyPickerDialogModel` props, the presentation builder mirrors the open flag into a local
/// signal and shapes the board, and this renders the reusable `WarcraftDialog` directly,
/// handing it the isolated key board body region. The headless dialog owns the styled content
/// box and the title/close header; the board keyboard-dismiss rides in the body region.
#[component]
pub fn KeyPickerDialog(props: KeyPickerDialogModel) -> Element {
    let KeyPickerDialogPresentation {
        open,
        title,
        columns,
        on_pick,
        on_close,
        on_open_change,
    } = use_key_picker_dialog_presentation(&props);
    let body = KeyPickerBodyView {
        columns,
        on_pick,
        on_close,
    };
    rsx! {
        WarcraftDialog::<KeyPickerBodyView, Empty> {
            title,
            body,
            open,
            on_open_change,
        }
    }
}

assert_component!(KeyPickerDialog);
