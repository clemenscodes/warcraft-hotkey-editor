pub mod components;
mod props;

use dioxus::prelude::*;

use components::alt_position_picker::{AltPositionPicker, AltPositionPickerProps};

pub use props::TileOverrideAltPickerProps;

/// Mounts the off-state position picker only while it is open, reading its own open
/// signal to decide — the way `CarriersDialogHost` reads its open state.
#[component]
pub fn TileOverrideAltPicker(props: TileOverrideAltPickerProps) -> Element {
    if !*props.alt_position_picker_open.read() {
        return rsx! {};
    }
    rsx! {
        AltPositionPicker { ..AltPositionPickerProps::from(&props) }
    }
}
