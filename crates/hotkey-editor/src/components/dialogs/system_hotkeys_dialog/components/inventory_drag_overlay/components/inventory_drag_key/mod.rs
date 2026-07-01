mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::InventoryDragKeyProps;

assert_component!(InventoryDragKey);

/// The key glyph shown on the inventory drag follower.
#[component]
pub fn InventoryDragKey(props: InventoryDragKeyProps) -> Element {
    let label = props.label;
    rsx! {
        div {
            class: CLASS,
            {label}
        }
    }
}
