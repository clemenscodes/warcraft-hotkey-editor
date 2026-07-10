mod props;
mod style;

use super::shared::slot_contents::SlotContents;
use dioxus::prelude::*;
pub use props::HighlightedSlotProps;
use style::CLASS;
use tw_macro::assert_component;

/// The highlighted look of a system slot: the gold border-image frame with a gold
/// glow overlay, composing the shared slot content. Presentational — the dispatcher
/// builds its props and renders it when the slot's glow state is highlighted.
#[component]
pub fn HighlightedSlot(props: HighlightedSlotProps) -> Element {
    let contents = props.contents;
    rsx! {
        div {
            class: CLASS,
            SlotContents { ..contents }
        }
    }
}

assert_component!(HighlightedSlot);
