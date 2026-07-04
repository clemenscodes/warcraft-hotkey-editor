mod data;
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
    let count = props.count;
    let noun = if count == 1 {
        data::SINGULAR
    } else {
        data::PLURAL
    };
    rsx! {
        span {
            class: CLASS,
            "{count} {noun}"
        }
    }
}
