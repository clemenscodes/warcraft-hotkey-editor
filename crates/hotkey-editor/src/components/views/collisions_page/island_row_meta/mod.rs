mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::IslandRowMetaProps;
use style::CLASS;
assert_component!(IslandRowMeta);

/// The text column of a collision card.
#[component]
pub fn IslandRowMeta(props: IslandRowMetaProps) -> Element {
    let children = props.children;
    rsx! {
        div {
            class: CLASS,
            {children}
        }
    }
}
