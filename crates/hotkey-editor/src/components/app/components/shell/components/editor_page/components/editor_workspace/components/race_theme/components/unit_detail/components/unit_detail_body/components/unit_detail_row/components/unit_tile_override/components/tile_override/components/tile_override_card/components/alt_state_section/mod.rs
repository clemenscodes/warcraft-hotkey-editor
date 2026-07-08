pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_label::{AltStateLabel, AltStateLabelProps};
use components::alt_state_controls::{AltStateControls, AltStateControlsProps};
use components::alt_state_line::AltStateLine;
use dioxus::prelude::*;
pub use props::AltStateSectionProps;
use style::{CONTAINER, HEADER, HEADER_TEXT};
use tw_macro::assert_component;
assert_component!(AltStateSection);

/// The off-state block of a tile override: its label and controls over any
/// description lines. It owns its own block, header, and label-column elements
/// directly. Renders nothing when the tile has no alternate state.
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
        div {
            class: CONTAINER,
            div {
                class: HEADER,
                div {
                    class: HEADER_TEXT,
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
