pub mod components;
mod logic;
mod props;
mod state;

use components::conflict_slot::{ConflictSlot, ConflictSlotProps};
use components::highlighted_slot::{HighlightedSlot, HighlightedSlotProps};
use components::idle_slot::{IdleSlot, IdleSlotProps};
use dioxus::prelude::*;
pub use props::SystemSlotProps;
pub use state::SystemSlotState;
use tw_macro::assert_component;

/// The framed WC3 slot cell shared by the inventory grid and the hero/control-group
/// layouts. A pure dispatcher: from the slot's glow state it renders the matching
/// look — `IdleSlot` xor `HighlightedSlot` xor `ConflictSlot`. Each look owns its
/// own framed root and glow; this dispatcher only builds each look's props from the
/// shared `SystemSlotProps` and renders the one the state selects. Purely
/// presentational — the host owns the outer interactive element, size, and behaviour.
#[component]
pub fn SystemSlot(props: SystemSlotProps) -> Element {
    match props.state {
        SystemSlotState::Idle => {
            let idle = IdleSlotProps::from(&props);
            rsx! {
                IdleSlot { ..idle }
            }
        }
        SystemSlotState::Highlighted => {
            let highlighted = HighlightedSlotProps::from(&props);
            rsx! {
                HighlightedSlot { ..highlighted }
            }
        }
        SystemSlotState::Conflict => {
            let conflict = ConflictSlotProps::from(&props);
            rsx! {
                ConflictSlot { ..conflict }
            }
        }
    }
}

assert_component!(SystemSlot);
