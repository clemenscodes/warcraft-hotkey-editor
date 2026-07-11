pub mod components;
mod model;
mod view;

pub use view::TileOverrideAltPickerView;

use dioxus::prelude::*;

use components::alt_position_picker::AltPositionPicker;
use tw_macro::assert_component;

use model::TileOverrideAltPickerModel;

/// Mounts the off-state position picker only while it is open, reading its own open
/// signal to decide — the way `CarriersDialogHost` reads its open state.
#[component]
pub fn TileOverrideAltPicker(props: TileOverrideAltPickerModel) -> Element {
    if !*props.alt_position_picker_open.read() {
        return rsx! {};
    }
    let TileOverrideAltPickerModel {
        object_id,
        display_name,
        picker_slots,
        alt_position_picker_open,
    } = props;
    rsx! {
        AltPositionPicker {
            object_id,
            display_name,
            picker_slots,
            alt_position_picker_open,
        }
    }
}

assert_component!(TileOverrideAltPicker);
