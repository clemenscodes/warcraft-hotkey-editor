mod props;

use dioxus::prelude::*;

use super::position_picker::{AltPositionPicker, AltPositionPickerProps};

pub use props::TileOverrideAltPickerProps;

/// Mounts the off-state position picker only while it is open.
#[component]
pub fn TileOverrideAltPicker(props: TileOverrideAltPickerProps) -> Element {
    if !props.visible {
        return rsx! {};
    }
    rsx! {
        AltPositionPicker { ..AltPositionPickerProps::from(&props) }
    }
}
