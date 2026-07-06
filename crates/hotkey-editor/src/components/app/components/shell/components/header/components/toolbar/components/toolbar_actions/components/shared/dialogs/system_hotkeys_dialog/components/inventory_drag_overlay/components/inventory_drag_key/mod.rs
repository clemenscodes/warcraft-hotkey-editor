mod props;
mod style;

use dioxus::prelude::*;
pub use props::InventoryDragKeyProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(InventoryDragKey);

/// The key glyph shown on the inventory drag follower.
#[component]
pub fn InventoryDragKey(props: InventoryDragKeyProps) -> Element {
    let label = props.label;
    rsx! {
        div { class: CLASS, {label} }
    }
}
