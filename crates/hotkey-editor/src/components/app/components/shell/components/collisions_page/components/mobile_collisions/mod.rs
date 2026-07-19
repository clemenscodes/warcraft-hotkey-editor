pub mod components;
mod model;
mod view;

pub use view::MobileCollisionsView;
mod style;

use components::collision_kind_nav::CollisionKindNav;
use components::collision_pager::CollisionPager;
use dioxus::prelude::*;
use model::MobileCollisionsModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MobileCollisions(props: MobileCollisionsModel) -> Element {
    let breadcrumbs = props.breadcrumbs;
    let content = props.content;
    rsx! {
        div {
            class: CLASS,
            CollisionKindNav {
                breadcrumbs,
            }
            CollisionPager {
                content,
            }
        }
    }
}

assert_component!(MobileCollisions);
