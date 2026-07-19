pub mod components;
mod model;
mod presentation;
mod view;

pub use view::CollisionPagerView;
mod style;

use components::collision_pager_cards::CollisionPagerCards;
use dioxus::prelude::*;
use model::CollisionPagerModel;
use presentation::{CollisionPagerPresentation, use_collision_pager};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CollisionPager(props: CollisionPagerModel) -> Element {
    let presentation = use_collision_pager(&props);
    let CollisionPagerPresentation {
        onmounted,
        onscrollend,
        content,
    } = presentation;
    rsx! {
        section {
            class: CLASS,
            aria_label: "Collision pager",
            onmounted: move |event| onmounted.call(event),
            onscrollend: move |event| onscrollend.call(event),
            CollisionPagerCards {
                content,
            }
        }
    }
}

assert_component!(CollisionPager);
