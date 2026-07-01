mod props;

use dioxus::prelude::*;

use crate::components::tile_override::components::tile_override_card::components::alt_state_position_button::{
    AltStatePositionButton, AltStatePositionButtonProps,
};

use crate::components::tile_override::components::tile_override_card::components::override_key_cell::{
    OverrideKeyCell, OverrideKeyCellProps,
};

pub use props::AltStateControlsProps;

/// The position button and off-state hotkey cell of the alt-state block; renders
/// nothing when the off-state is not editable in this context.
#[component]
pub fn AltStateControls(props: AltStateControlsProps) -> Element {
    if !props.show {
        return rsx! {};
    }
    let position_button = AltStatePositionButtonProps::from(&props);
    let key_cell = OverrideKeyCellProps::from(&props);
    rsx! {
        AltStatePositionButton { ..position_button }
        OverrideKeyCell { ..key_cell }
    }
}
