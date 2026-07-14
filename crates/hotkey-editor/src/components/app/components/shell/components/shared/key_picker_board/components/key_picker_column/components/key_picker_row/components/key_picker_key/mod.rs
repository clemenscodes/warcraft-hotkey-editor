pub mod components;
mod model;
mod presentation;
mod view;

pub use view::KeyPickerKeyView;

use crate::components::app::components::shell::components::shared::key_picker_board::KeyWidth;
use components::narrow_key_slot::NarrowKeySlot;
use components::wide_key_slot::WideKeySlot;
use dioxus::prelude::*;
use model::KeyPickerKeyModel;
use presentation::KeyPickerKeyPresentation;
use tw_macro::assert_component;

#[component]
pub fn KeyPickerKey(props: KeyPickerKeyModel) -> Element {
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
