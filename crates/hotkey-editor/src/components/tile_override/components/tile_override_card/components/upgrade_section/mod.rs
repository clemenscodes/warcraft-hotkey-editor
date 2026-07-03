mod props;

use dioxus::prelude::*;

use crate::components::tile_override::components::tile_override_card::components::shared::alt_state_container::AltStateContainer;
use crate::components::tile_override::components::tile_override_card::components::shared::alt_state_header::AltStateHeader;
use crate::components::tile_override::components::tile_override_card::components::shared::alt_state_header_text::AltStateHeaderText;
use crate::components::tile_override::components::tile_override_card::components::shared::alt_state_label::{AltStateLabel, AltStateLabelProps};
use crate::components::tile_override::components::tile_override_card::components::shared::alt_state_position_button::{AltStatePositionButton, AltStatePositionButtonProps};
use crate::components::tile_override::components::tile_override_card::components::shared::override_key_cell::{OverrideKeyCell, OverrideKeyCellProps};

pub use props::UpgradeSectionProps;

#[component]
pub fn UpgradeSection(props: UpgradeSectionProps) -> Element {
    if !props.show {
        return rsx! {};
    }
    let label = AltStateLabelProps::from(&props);
    let position_button = AltStatePositionButtonProps::from(&props);
    let key_cell = OverrideKeyCellProps::from(&props);
    rsx! {
        AltStateContainer {
            AltStateHeader {
                AltStateHeaderText {
                    AltStateLabel { ..label }
                }
                AltStatePositionButton { ..position_button }
                OverrideKeyCell { ..key_cell }
            }
        }
    }
}
