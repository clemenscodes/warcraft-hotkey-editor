mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::TileIconProps;
use style::CLASS;
assert_component!(TileIcon);

/// The ability icon, filling the tile. Rendered only when the occupant has an
/// icon; an iconless occupant falls back to the sibling `TileLabel`.
#[component]
pub fn TileIcon(props: TileIconProps) -> Element {
    let Some(src) = props.src else {
        return rsx! {};
    };
    let alt = props.alt;
    rsx! {
        img {
            class: CLASS,
            src,
            alt,
            loading: "lazy",
            decoding: "async",
        }
    }
}
