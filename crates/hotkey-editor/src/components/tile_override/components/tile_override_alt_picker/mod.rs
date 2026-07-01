mod props;

use dioxus::prelude::*;

use super::position_picker::AltPositionPicker;

pub use props::TileOverrideAltPickerProps;

/// Mounts the off-state position picker only while it is open.
#[component]
pub fn TileOverrideAltPicker(props: TileOverrideAltPickerProps) -> Element {
    if !props.visible {
        return rsx! {};
    }
    rsx! {
        AltPositionPicker {
            object_id: props.object_id,
            display_name: props.display_name,
            picker_slots: props.picker_slots,
            loaded_keys: props.loaded_keys,
            grid_layout: props.grid_layout,
            dragging_slot: props.dragging_slot,
            drop_target_tile: props.drop_target_tile,
            drag_follower: props.drag_follower,
            alt_position_picker_open: props.alt_position_picker_open,
        }
    }
}
