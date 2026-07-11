mod data;
mod model;
mod view;

pub use view::CollisionCountView;
mod style;

use dioxus::prelude::*;
use model::CollisionCountModel;
use style::CLASS;
use tw_macro::assert_component;

/// The collision-count line on a collision card.
#[component]
pub fn CollisionCount(props: CollisionCountModel) -> Element {
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

assert_component!(CollisionCount);
