mod props;
mod style;

use super::shared::slot_contents::SlotContents;
use dioxus::prelude::*;
pub use props::IdleSlotProps;
use style::CLASS;
use tw_macro::assert_component;

/// The idle (unglowed) look of a system slot: the gold border-image frame composing
/// the shared slot content. Presentational — the dispatcher builds its props and
/// renders it when the slot's glow state is idle.
#[component]
pub fn IdleSlot(props: IdleSlotProps) -> Element {
    let contents = props.contents;
    rsx! {
        div {
            class: CLASS,
            SlotContents { ..contents }
        }
    }
}

assert_component!(IdleSlot);
