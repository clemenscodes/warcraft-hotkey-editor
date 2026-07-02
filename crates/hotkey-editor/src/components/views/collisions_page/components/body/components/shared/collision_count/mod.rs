mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::CollisionCountProps;
use style::CLASS;
assert_component!(CollisionCount);

/// The collision-count line on a collision card.
#[component]
pub fn CollisionCount(props: CollisionCountProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}
