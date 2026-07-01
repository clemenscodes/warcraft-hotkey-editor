mod props;

use dioxus::prelude::*;

use super::alt_state_container::AltStateContainer;
use super::alt_state_header::AltStateHeader;
use super::alt_state_header_text::AltStateHeaderText;
use super::alt_state_label::{AltStateLabel, AltStateLabelProps};
use super::alt_state_position_button::{AltStatePositionButton, AltStatePositionButtonProps};
use super::override_key_cell::{OverrideKeyCell, OverrideKeyCellProps};

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
