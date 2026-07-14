pub mod components;
mod model;
mod view;

pub use view::AltStateSectionView;
mod style;

use components::alt_state_header::AltStateHeader;
use components::alt_state_line::AltStateLine;
use dioxus::prelude::*;
use model::AltStateSectionModel;
use style::CLASS;
use tw_macro::assert_component;

/// The off-state block of a tile override: its header (label and controls) over any
/// description lines. It owns its own block directly and delegates the header row to its
/// child. Renders nothing when the tile has no alternate state.
#[component]
pub fn AltStateSection(props: AltStateSectionModel) -> Element {
    let AltStateSectionModel {
        alt_name_text,
        alt_description_lines,
        show_alt_controls,
        alt_hotkey_label,
        alt_hotkey_is_editing,
        alt_hotkey_is_special_token,
        on_position_click,
        on_hotkey_activate,
    } = props;
    let has_alt_state = alt_name_text.is_some() || !alt_description_lines.is_empty();
    if !has_alt_state {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
            AltStateHeader {
                alt_name_text,
                show: show_alt_controls,
                hotkey_label: alt_hotkey_label,
                is_editing: alt_hotkey_is_editing,
                is_special: alt_hotkey_is_special_token,
                on_position_click,
                on_hotkey_activate,
            }
            for text in alt_description_lines {
                AltStateLine {
                    text,
                }
            }
        }
    }
}

assert_component!(AltStateSection);
