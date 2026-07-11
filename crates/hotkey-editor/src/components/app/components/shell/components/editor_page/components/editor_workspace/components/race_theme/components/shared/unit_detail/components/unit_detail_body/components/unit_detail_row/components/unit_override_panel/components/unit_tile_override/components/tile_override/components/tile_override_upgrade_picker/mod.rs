pub mod components;
mod model;
mod view;

pub use view::TileOverrideUpgradePickerView;

use dioxus::prelude::*;

use components::upgrade_position_picker::UpgradePositionPicker;
use tw_macro::assert_component;

use model::TileOverrideUpgradePickerModel;

/// Renders the upgraded-form position picker only when the ability has an upgraded
/// form; the picker's own dialog handles open/closed.
#[component]
pub fn TileOverrideUpgradePicker(props: TileOverrideUpgradePickerModel) -> Element {
    if !*props.upgrade_position_picker_open.read() || props.upgrade_unit_id.is_none() {
        return rsx! {};
    }
    let upgrade_unit_id = props
        .upgrade_unit_id
        .expect("guarded to Some before render");
    let TileOverrideUpgradePickerModel {
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
