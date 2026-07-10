mod props;
mod view;

pub use view::TileIconView;
mod style;

use dioxus::prelude::*;
use props::TileIconProps;
use style::CLASS;
use tw_macro::assert_component;

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
            // Images are `draggable` by default; a native HTML5 image drag fires
            // `dragstart` on pointerdown and pre-empts the tile's own pointer-based drag,
            // so a grid rearrange would silently do nothing. Cancel the native drag.
            ondragstart: move |event| event.prevent_default(),
            loading: "lazy",
            decoding: "async",
        }
    }
}

assert_component!(TileIcon);
