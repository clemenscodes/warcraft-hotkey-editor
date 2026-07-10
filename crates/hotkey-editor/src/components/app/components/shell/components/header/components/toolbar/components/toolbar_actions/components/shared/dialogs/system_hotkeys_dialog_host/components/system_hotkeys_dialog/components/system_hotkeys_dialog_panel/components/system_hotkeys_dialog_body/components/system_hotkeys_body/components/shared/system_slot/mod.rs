pub mod components;
mod props;
mod state;

use components::conflict_slot::ConflictSlot;
use components::highlighted_slot::HighlightedSlot;
use components::idle_slot::IdleSlot;
use dioxus::prelude::*;
use props::SystemSlotProps;
pub use state::SystemSlotState;
use tw_macro::assert_component;

/// The framed WC3 slot cell shared by the inventory grid and the hero/control-group
/// layouts. A pure dispatcher: from the slot's glow state it renders the matching
/// look — `IdleSlot` xor `HighlightedSlot` xor `ConflictSlot`. Each look owns its
/// own framed root and glow; this dispatcher only forwards the shared slot data to
/// the one the state selects. Purely presentational — the host owns the outer
/// interactive element, size, and behaviour.
#[component]
pub fn SystemSlot(props: SystemSlotProps) -> Element {
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
