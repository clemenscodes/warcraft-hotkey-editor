mod model;
mod view;

pub use view::CollisionsBreadcrumbsView;

use crate::components::app::components::shell::components::shared::breadcrumbs::Breadcrumbs;
use dioxus::prelude::*;
use model::CollisionsBreadcrumbsModel;
use tw_macro::assert_component;

#[component]
pub fn CollisionsBreadcrumbs(props: CollisionsBreadcrumbsModel) -> Element {
    let breadcrumbs = props.breadcrumbs;
    rsx! {
        Breadcrumbs {
            breadcrumbs,
            aria_label: "Collision categories",
        }
    }
}

assert_component!(CollisionsBreadcrumbs);
