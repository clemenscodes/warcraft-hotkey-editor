pub mod components;
mod model;
mod presentation;
mod style;
mod view;

pub use view::HotkeyAltPositionPickerDialogView;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::hotkey_alt_position_picker_dialog_body::HotkeyAltPositionPickerDialogBodyView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use model::HotkeyAltPositionPickerDialogModel;
use presentation::OpenHotkeyAltPositionPickerDialog;
use presentation::use_hotkey_alt_position_picker_dialog;
use style::CLASS;
use tw_macro::assert_component;

/// Mounts the off-state position picker only while it is open, reading its own open signal
/// to decide — the way `CarriersDialog` reads its open state. It renders the reusable
/// `WarcraftDialog`, handing it the isolated position-picker grid as its body region;
/// keeping the mount conditional here re-initialises the headless dialog each time it opens.
#[component]
pub fn HotkeyAltPositionPickerDialog(props: HotkeyAltPositionPickerDialogModel) -> Element {
    let dialog = use_hotkey_alt_position_picker_dialog(&props);
    let Some(dialog) = dialog else {
        return rsx! {};
    };
    let OpenHotkeyAltPositionPickerDialog {
        title,
        on_open_change,
    } = dialog;
    let object_id = props.object_id;
    let picker_slots = props.picker_slots.clone();
    let body = HotkeyAltPositionPickerDialogBodyView {
        object_id,
        picker_slots,
    };
    rsx! {
        div {
            class: CLASS,
            WarcraftDialog::<HotkeyAltPositionPickerDialogBodyView,Empty> {
                title,
                body,
                open: true,
                on_open_change,
            }
        }
    }
}

assert_component!(HotkeyAltPositionPickerDialog);
