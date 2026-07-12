pub mod components;
mod model;
mod view;

pub use view::HotkeyAltPositionPickerDialogHostView;

use dioxus::prelude::*;

use components::hotkey_alt_position_picker_dialog::HotkeyAltPositionPickerDialog;
use tw_macro::assert_component;

use model::HotkeyAltPositionPickerDialogHostModel;

/// Mounts the off-state position picker only while it is open, reading its own open
/// signal to decide — the way `CarriersDialogHost` reads its open state.
#[component]
pub fn HotkeyAltPositionPickerDialogHost(props: HotkeyAltPositionPickerDialogHostModel) -> Element {
    if !*props.hotkey_alt_position_picker_open.read() {
        return rsx! {};
    }
    let HotkeyAltPositionPickerDialogHostModel {
        object_id,
        display_name,
        picker_slots,
        hotkey_alt_position_picker_open,
    } = props;
    rsx! {
        HotkeyAltPositionPickerDialog {
            object_id,
            display_name,
            picker_slots,
            hotkey_alt_position_picker_open,
        }
    }
}

assert_component!(HotkeyAltPositionPickerDialogHost);
