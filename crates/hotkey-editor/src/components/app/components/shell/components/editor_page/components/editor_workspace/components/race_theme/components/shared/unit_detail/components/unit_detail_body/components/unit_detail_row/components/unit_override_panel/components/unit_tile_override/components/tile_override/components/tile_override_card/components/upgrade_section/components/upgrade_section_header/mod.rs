pub mod components;
mod props;
mod style;

use components::upgrade_section_header_label_column::{
    UpgradeSectionHeaderLabelColumn, UpgradeSectionHeaderLabelColumnProps,
};
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_position_button::{AltStatePositionButton, AltStatePositionButtonProps};
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::override_key::{OverrideKey, OverrideKeyProps};
use dioxus::prelude::*;
pub use props::UpgradeSectionHeaderProps;
use style::CLASS;
use tw_macro::assert_component;

/// The upgraded-form block's top row: the label column beside the position button and
/// hotkey cell.
#[component]
pub fn UpgradeSectionHeader(props: UpgradeSectionHeaderProps) -> Element {
    let label_column = UpgradeSectionHeaderLabelColumnProps::from(&props);
    let position_button = AltStatePositionButtonProps::from(&props);
    let key_cell = OverrideKeyProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            UpgradeSectionHeaderLabelColumn { ..label_column }
            AltStatePositionButton { ..position_button }
            OverrideKey { ..key_cell }
        }
    }
}

assert_component!(UpgradeSectionHeader);
