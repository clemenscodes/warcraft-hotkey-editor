pub mod components;
mod model;
mod view;

pub use view::HotkeyUpgradePositionPickerDialogHostView;

use dioxus::prelude::*;

use components::hotkey_upgrade_position_picker_dialog::HotkeyUpgradePositionPickerDialog;
use tw_macro::assert_component;

use model::HotkeyUpgradePositionPickerDialogHostModel;

/// Renders the upgraded-form position picker only when the ability has an upgraded
/// form; the picker's own dialog handles open/closed.
#[component]
pub fn HotkeyUpgradePositionPickerDialogHost(
    props: HotkeyUpgradePositionPickerDialogHostModel,
) -> Element {
    if !*props.hotkey_upgrade_position_picker_open.read() || props.upgrade_unit_id.is_none() {
        return rsx! {};
    }
    let upgrade_unit_id = props
        .upgrade_unit_id
        .expect("guarded to Some before render");
    let HotkeyUpgradePositionPickerDialogHostModel {
        display_name,
        picker_slots,
        hotkey_upgrade_position_picker_open,
        ..
    } = props;
    rsx! {
        HotkeyUpgradePositionPickerDialog {
            upgrade_unit_id,
            display_name,
            picker_slots,
            hotkey_upgrade_position_picker_open,
        }
    }
}

assert_component!(HotkeyUpgradePositionPickerDialogHost);
