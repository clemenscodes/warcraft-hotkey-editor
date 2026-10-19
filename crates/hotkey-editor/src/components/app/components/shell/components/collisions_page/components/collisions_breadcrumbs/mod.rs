mod model;
mod view;

pub use view::CollisionsBreadcrumbsView;

use crate::components::app::components::shell::components::collisions_page::components::mobile_collisions::components::collision_kind_nav::CollisionKindNav;
use dioxus::prelude::*;
use model::CollisionsBreadcrumbsModel;
use tw_macro::assert_component;

#[component]
pub fn CollisionsBreadcrumbs(props: CollisionsBreadcrumbsModel) -> Element {
    let breadcrumbs = props.breadcrumbs;
    rsx! {
        CollisionKindNav {
            breadcrumbs,
        }
    }
}

assert_component!(CollisionsBreadcrumbs);
