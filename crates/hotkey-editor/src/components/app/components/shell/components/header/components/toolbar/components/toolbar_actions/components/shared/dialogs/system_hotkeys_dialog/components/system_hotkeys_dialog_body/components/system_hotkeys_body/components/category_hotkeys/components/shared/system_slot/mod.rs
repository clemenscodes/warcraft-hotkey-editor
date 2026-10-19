pub mod components;
mod model;
mod view;

pub use view::SystemSlotView;
mod state;

use components::conflict_slot::ConflictSlot;
use components::highlighted_slot::HighlightedSlot;
use components::idle_slot::IdleSlot;
use dioxus::prelude::*;
use model::SystemSlotModel;
pub use state::SystemSlotState;
use tw_macro::assert_component;

#[component]
pub fn SystemSlot(props: SystemSlotModel) -> Element {
    match props.state {
        SystemSlotState::Idle => {
            let slot_label = props.slot_label.clone();
            let key_label = props.key_label.clone();
            let conflict = props.conflict;
            let tooltip_text = props.tooltip_text.clone();
            let tooltip_placement = props.tooltip_placement;
            let dragging = props.dragging;
            rsx! {
                IdleSlot {
                    slot_label,
                    key_label,
                    conflict,
                    tooltip_text,
                    tooltip_placement,
                    dragging,
                }
            }
        }
        SystemSlotState::Highlighted => {
            let slot_label = props.slot_label.clone();
            let key_label = props.key_label.clone();
            let conflict = props.conflict;
            let tooltip_text = props.tooltip_text.clone();
            let tooltip_placement = props.tooltip_placement;
            let dragging = props.dragging;
            rsx! {
                HighlightedSlot {
                    slot_label,
                    key_label,
                    conflict,
                    tooltip_text,
                    tooltip_placement,
                    dragging,
                }
            }
        }
        SystemSlotState::Conflict => {
            let slot_label = props.slot_label.clone();
            let key_label = props.key_label.clone();
            let conflict = props.conflict;
            let tooltip_text = props.tooltip_text.clone();
            let tooltip_placement = props.tooltip_placement;
            let dragging = props.dragging;
            rsx! {
                ConflictSlot {
                    slot_label,
                    key_label,
                    conflict,
                    tooltip_text,
                    tooltip_placement,
                    dragging,
                }
            }
        }
    }
}

assert_component!(SystemSlot);
