mod props;
mod style;

use super::shared::slot_contents::SlotContents;
use dioxus::prelude::*;
pub use props::ConflictSlotProps;
use style::CLASS;
use tw_macro::assert_component;

/// The conflict look of a system slot: the gold border-image frame with a danger-red
/// glow overlay, composing the shared slot content. Presentational — the dispatcher
/// builds its props and renders it when the slot's glow state is a binding conflict.
#[component]
pub fn ConflictSlot(props: ConflictSlotProps) -> Element {
    let contents = props.contents;
    rsx! {
        div {
            class: CLASS,
            SlotContents { ..contents }
        }
    }
}

assert_component!(ConflictSlot);
