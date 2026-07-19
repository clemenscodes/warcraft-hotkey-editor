mod model;
mod view;

pub use view::CollisionKindTabCountView;
mod style;

use dioxus::prelude::*;
use model::CollisionKindTabCountModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CollisionKindTabCount(props: CollisionKindTabCountModel) -> Element {
    let count = props.count;
    rsx! {
        span {
            class: CLASS,
            "{count}"
        }
    }
}

assert_component!(CollisionKindTabCount);
