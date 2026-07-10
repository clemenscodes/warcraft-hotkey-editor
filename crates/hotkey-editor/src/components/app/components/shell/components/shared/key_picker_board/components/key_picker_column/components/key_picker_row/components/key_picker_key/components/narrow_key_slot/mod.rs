mod props;
mod style;

use crate::components::app::components::shell::components::shared::key_picker_board::components::key_picker_column::components::key_picker_row::components::key_picker_key::components::shared::color_key::ColorKey;
use dioxus::prelude::*;
use props::NarrowKeySlotProps;
use style::CLASS;
use tw_macro::assert_component;

/// The sizing box for a standard-width picker key. A component owns its look; its
/// parent owns its size — so the key's width lives here, and the color leaf inside
/// fills the box. Standard caps (single letters, digits) get this narrow width.
#[component]
pub fn NarrowKeySlot(props: NarrowKeySlotProps) -> Element {
    let state = props.state;
    let label = props.label;
    let disabled = props.disabled;
    let onclick = props.onclick;
    let tooltip_text = props.tooltip_text;
    let tooltip_placement = props.tooltip_placement;
    let tooltip_anchor = props.tooltip_anchor;
    rsx! {
        div { class: CLASS,
            ColorKey {
                state,
                label,
                disabled,
                onclick,
                tooltip_text,
                tooltip_placement,
                tooltip_anchor,
            }
        }
    }
}

assert_component!(NarrowKeySlot);
