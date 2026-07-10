mod props;
mod style;

use super::shared::slot_contents::SlotContents;
use dioxus::prelude::*;
use props::ConflictSlotProps;
use style::CLASS;
use tw_macro::assert_component;

/// The conflict look of a system slot: the gold border-image frame with a danger-red
/// glow overlay, composing the shared slot content. Presentational — the dispatcher
/// renders it when the slot's glow state is a binding conflict.
#[component]
pub fn ConflictSlot(props: ConflictSlotProps) -> Element {
    let slot_label = props.slot_label;
    let key_label = props.key_label;
    let conflict = props.conflict;
    let tooltip_text = props.tooltip_text;
    let tooltip_placement = props.tooltip_placement;
    let dragging = props.dragging;
    rsx! {
        div {
            class: CLASS,
            SlotContents {
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

assert_component!(ConflictSlot);
