mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::IslandCollisionCountProps;
use style::CLASS;
assert_component!(IslandCollisionCount);

/// The collision-count line on a collision card.
#[component]
pub fn IslandCollisionCount(props: IslandCollisionCountProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}
