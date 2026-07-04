mod props;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_position_button::{
    AltStatePositionButton, AltStatePositionButtonProps,
};

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::override_key_cell::{
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
