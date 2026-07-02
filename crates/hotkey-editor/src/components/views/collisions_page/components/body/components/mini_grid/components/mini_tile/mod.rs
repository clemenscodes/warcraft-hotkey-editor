mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::MiniTileProps;
use style::CLASS;
assert_component!(MiniTile);

/// One tile of the mini grid; marked when it is the highlighted coordinate's tile.
#[component]
pub fn MiniTile(props: MiniTileProps) -> Element {
    let is_highlighted = props.is_highlighted;
    rsx! {
        div {
            class: CLASS,
            "data-highlighted": is_highlighted,
        }
    }
}
