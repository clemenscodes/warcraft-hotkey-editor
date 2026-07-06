mod data;
mod props;
mod style;

use dioxus::prelude::*;
pub use props::CollisionCountProps;
use style::CLASS;
use tw_macro::assert_component;
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
