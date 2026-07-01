pub mod components;
mod props;

use dioxus::prelude::*;

use super::alt_state_container::AltStateContainer;
use super::alt_state_header::AltStateHeader;
use super::alt_state_header_text::AltStateHeaderText;
use super::alt_state_label::{AltStateLabel, AltStateLabelProps};
use components::alt_state_controls::{AltStateControls, AltStateControlsProps};
use components::alt_state_line::AltStateLine;

pub use props::AltStateSectionProps;

#[component]
pub fn AltStateSection(props: AltStateSectionProps) -> Element {
    let has_alt_state = props.alt_name_text.is_some() || !props.alt_description_lines.is_empty();
    if !has_alt_state {
        return rsx! {};
    }
    let label = AltStateLabelProps::from(&props);
    let controls = AltStateControlsProps::from(&props);
    let description_lines = props.alt_description_lines;
    rsx! {
        AltStateContainer {
            AltStateHeader {
                AltStateHeaderText {
                    AltStateLabel { ..label }
                }
                AltStateControls { ..controls }
            }
            for text in description_lines {
                AltStateLine { text }
            }
        }
    }
}
