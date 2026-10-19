pub mod components;
mod model;
mod view;

pub use view::CollisionKindNavView;
mod style;

use components::collision_kind_tab::CollisionKindTab;
use dioxus::prelude::*;
use model::CollisionKindNavModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CollisionKindNav(props: CollisionKindNavModel) -> Element {
    let breadcrumbs = props.breadcrumbs;
    rsx! {
        nav {
            class: CLASS,
            aria_label: "Collision kinds",
            for breadcrumb in breadcrumbs {
                CollisionKindTab {
                    key: "{breadcrumb.label}",
                    label: breadcrumb.label,
                    count: breadcrumb.count,
                    active: breadcrumb.active,
                    onclick: breadcrumb.onclick,
                }
            }
        }
    }
}

assert_component!(CollisionKindNav);
