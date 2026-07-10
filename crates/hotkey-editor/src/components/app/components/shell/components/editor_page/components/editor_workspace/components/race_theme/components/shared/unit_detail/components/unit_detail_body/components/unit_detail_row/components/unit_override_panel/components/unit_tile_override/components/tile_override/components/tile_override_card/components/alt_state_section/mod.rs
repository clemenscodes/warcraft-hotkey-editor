pub mod components;
mod props;
mod style;

use components::alt_state_header::{AltStateHeader, AltStateHeaderProps};
use components::alt_state_line::AltStateLine;
use dioxus::prelude::*;
pub use props::AltStateSectionProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(AltStateSection);

/// The off-state block of a tile override: its header (label and controls) over any
/// description lines. It owns its own block directly and delegates the header row to its
/// child. Renders nothing when the tile has no alternate state.
#[component]
pub fn AltStateSection(props: AltStateSectionProps) -> Element {
    let has_alt_state = props.alt_name_text.is_some() || !props.alt_description_lines.is_empty();
    if !has_alt_state {
        return rsx! {};
    }
    let header = AltStateHeaderProps::from(&props);
    let description_lines = props.alt_description_lines;
    rsx! {
        div {
            class: CLASS,
            AltStateHeader { ..header }
            for text in description_lines {
                AltStateLine { text }
            }
        }
    }
}
