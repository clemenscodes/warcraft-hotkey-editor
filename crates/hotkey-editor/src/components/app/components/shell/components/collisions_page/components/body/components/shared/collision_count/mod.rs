mod data;
mod model;
mod presentation;
mod view;

pub use view::CollisionCountView;
mod style;

use dioxus::prelude::*;
use model::CollisionCountModel;
use presentation::CollisionCountPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CollisionCount(props: CollisionCountModel) -> Element {
    let CollisionCountPresentation { count, noun } = CollisionCountPresentation::from(&props);
    rsx! {
        span {
            class: CLASS,
            "{count} {noun}"
        }
    }
}

assert_component!(CollisionCount);
