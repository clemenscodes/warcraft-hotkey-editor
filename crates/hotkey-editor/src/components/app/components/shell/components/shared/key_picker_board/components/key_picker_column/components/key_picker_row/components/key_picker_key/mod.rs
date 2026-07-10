pub mod components;
mod logic;
mod props;

use crate::components::app::components::shell::components::shared::key_picker_board::KeyWidth;
use components::narrow_key_slot::NarrowKeySlot;
use components::wide_key_slot::WideKeySlot;
use dioxus::prelude::*;
use logic::KeyPickerKeyPresentation;
use props::KeyPickerKeyProps;
use tw_macro::assert_component;

/// A single key on the picker board: an on-screen keyboard button that assigns its key
/// when clicked. It carries no look of its own — it is the dispatcher that derives the
/// key's width from its cell and renders the matching sizing slot (`NarrowKeySlot` xor
/// `WideKeySlot`). Each slot owns the key's width and fills it with the color leaf that
/// owns the look. The body only chooses the width; the color is chosen further down.
#[component]
pub fn KeyPickerKey(props: KeyPickerKeyProps) -> Element {
    let presentation = KeyPickerKeyPresentation::from(&props);
    let state = presentation.state;
    let label = presentation.label;
    let disabled = presentation.disabled;
    let onclick = presentation.onclick;
    let tooltip_text = presentation.tooltip_text;
    let tooltip_placement = presentation.tooltip_placement;
    let tooltip_anchor = presentation.tooltip_anchor;
    match presentation.width {
        KeyWidth::Standard => {
            rsx! {
                NarrowKeySlot {
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
        KeyWidth::Wide => {
            rsx! {
                WideKeySlot {
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
}

assert_component!(KeyPickerKey);
