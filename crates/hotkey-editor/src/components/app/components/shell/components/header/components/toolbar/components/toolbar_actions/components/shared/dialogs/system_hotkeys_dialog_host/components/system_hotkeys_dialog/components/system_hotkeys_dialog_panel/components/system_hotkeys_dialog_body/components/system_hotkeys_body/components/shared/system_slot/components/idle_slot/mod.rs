mod props;
mod view;

pub use view::IdleSlotView;
mod style;

use super::shared::slot_contents::SlotContents;
use dioxus::prelude::*;
use props::IdleSlotProps;
use style::CLASS;
use tw_macro::assert_component;

/// The idle (unglowed) look of a system slot: the gold border-image frame composing
/// the shared slot content. Presentational — the dispatcher renders it when the slot's
/// glow state is idle.
#[component]
pub fn IdleSlot(props: IdleSlotProps) -> Element {
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

assert_component!(IdleSlot);
