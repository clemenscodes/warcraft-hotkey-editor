pub mod components;
mod model;
mod view;

pub use view::CollisionSidebarView;
mod style;

use components::collision_list_scroll::CollisionListScroll;
use dioxus::prelude::*;
use model::CollisionSidebarModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CollisionSidebar(props: CollisionSidebarModel) -> Element {
    let cards = props.cards;
    rsx! {
        aside {
            class: CLASS,
            CollisionListScroll {
                cards,
            }
        }
    }
}

assert_component!(CollisionSidebar);
