pub mod components;
mod model;
mod presentation;
mod style;
mod view;

pub use view::HotkeyUpgradePositionPickerDialogView;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::hotkey_upgrade_position_picker_body::HotkeyUpgradePositionPickerBodyView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use model::HotkeyUpgradePositionPickerDialogModel;
use presentation::OpenHotkeyUpgradePositionPickerDialog;
use presentation::use_hotkey_upgrade_position_picker_dialog;
use style::CLASS;
use tw_macro::assert_component;

/// Connected wrapper for the upgraded-form position picker: mounts it only while the
/// picker is open and the ability has an upgraded form, resolving the dialog through the
/// host builder. It renders the reusable `WarcraftDialog`, handing it the isolated
/// upgraded-form picker body as its body region; keeping the mount conditional here
/// re-initialises the headless dialog each time it opens.
#[component]
pub fn HotkeyUpgradePositionPickerDialog(props: HotkeyUpgradePositionPickerDialogModel) -> Element {
    let dialog = use_hotkey_upgrade_position_picker_dialog(&props);
    let Some(dialog) = dialog else {
        return rsx! {};
    };
    let OpenHotkeyUpgradePositionPickerDialog {
        title,
        upgrade_unit_id,
        picker_slots,
        on_open_change,
    } = dialog;
    let body = HotkeyUpgradePositionPickerBodyView {
        upgrade_unit_id,
        picker_slots,
    };
    rsx! {
        div {
            class: CLASS,
            WarcraftDialog::<HotkeyUpgradePositionPickerBodyView, Empty> {
                title,
                body,
                open: true,
                on_open_change,
            }
        }
    }
}

assert_component!(HotkeyUpgradePositionPickerDialog);
