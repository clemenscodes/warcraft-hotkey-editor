mod props;

use dioxus::prelude::*;

use super::position_picker::UpgradePositionPicker;

pub use props::TileOverrideUpgradePickerProps;

/// Renders the upgraded-form position picker only when the ability has an upgraded
/// form; the picker's own dialog handles open/closed.
#[component]
pub fn TileOverrideUpgradePicker(props: TileOverrideUpgradePickerProps) -> Element {
    let Some(upgrade_unit_id) = props.upgrade_unit_id else {
        return rsx! {};
    };
    if !props.visible {
        return rsx! {};
    }
    rsx! {
        UpgradePositionPicker {
            upgrade_unit_id,
            display_name: props.display_name,
            picker_slots: props.picker_slots,
            loaded_keys: props.loaded_keys,
            grid_layout: props.grid_layout,
            dragging_slot: props.dragging_slot,
            drop_target_tile: props.drop_target_tile,
            drag_follower: props.drag_follower,
            upgrade_position_picker_open: props.upgrade_position_picker_open,
        }
    }
}
