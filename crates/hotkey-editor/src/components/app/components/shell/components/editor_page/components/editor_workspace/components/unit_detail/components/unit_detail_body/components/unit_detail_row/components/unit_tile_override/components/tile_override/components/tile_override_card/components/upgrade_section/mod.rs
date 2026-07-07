mod props;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_container::AltStateContainer;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_header::AltStateHeader;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_header_text::AltStateHeaderText;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_label::{AltStateLabel, AltStateLabelProps};
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_position_button::{AltStatePositionButton, AltStatePositionButtonProps};
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::override_key::{OverrideKey, OverrideKeyProps};

pub use props::UpgradeSectionProps;

#[component]
pub fn UpgradeSection(props: UpgradeSectionProps) -> Element {
    if !props.show {
        return rsx! {};
    }
    let label = AltStateLabelProps::from(&props);
    let position_button = AltStatePositionButtonProps::from(&props);
    let key_cell = OverrideKeyProps::from(&props);
    rsx! {
        AltStateContainer {
            AltStateHeader {
                AltStateHeaderText {
                    AltStateLabel { ..label }
                }
                AltStatePositionButton { ..position_button }
                OverrideKey { ..key_cell }
            }
        }
    }
}
