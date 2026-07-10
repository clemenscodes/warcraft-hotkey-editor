pub mod components;
mod props;
mod view;

pub use view::TileOverrideUpgradePickerView;

use dioxus::prelude::*;

use components::upgrade_position_picker::UpgradePositionPicker;
use tw_macro::assert_component;

use props::TileOverrideUpgradePickerProps;

/// Renders the upgraded-form position picker only when the ability has an upgraded
/// form; the picker's own dialog handles open/closed.
#[component]
pub fn TileOverrideUpgradePicker(props: TileOverrideUpgradePickerProps) -> Element {
    if !*props.upgrade_position_picker_open.read() || props.upgrade_unit_id.is_none() {
        return rsx! {};
    }
    let upgrade_unit_id = props
        .upgrade_unit_id
        .expect("guarded to Some before render");
    let TileOverrideUpgradePickerProps {
        display_name,
        picker_slots,
        upgrade_position_picker_open,
        ..
    } = props;
    rsx! {
        UpgradePositionPicker {
            upgrade_unit_id,
            display_name,
            picker_slots,
            upgrade_position_picker_open,
        }
    }
}

assert_component!(TileOverrideUpgradePicker);
