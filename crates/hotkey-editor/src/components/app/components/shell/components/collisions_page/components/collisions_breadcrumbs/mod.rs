mod model;
mod view;

pub use view::CollisionsBreadcrumbsView;

use crate::components::app::components::shell::components::shared::breadcrumbs::Breadcrumbs;
use dioxus::prelude::*;
use model::CollisionsBreadcrumbsModel;
use tw_macro::assert_component;

/// The collisions page's breadcrumb bar: the shared `Breadcrumbs` named for the collision
/// categories. Purely presentational — it forwards the prepared tabs its owning page hands
/// it and fixes the bar's assistive-tech name.
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
