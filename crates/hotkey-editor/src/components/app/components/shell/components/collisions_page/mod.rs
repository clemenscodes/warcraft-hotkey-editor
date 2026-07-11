pub mod components;
mod data;
mod model;
pub mod presentation;
mod view;

pub use view::CollisionsPageView;
mod style;

use crate::components::app::components::shell::components::shared::breadcrumbs::Breadcrumbs;
use components::body::Body;
use dioxus::prelude::*;
use model::CollisionsPageModel;
use presentation::use_collisions_page;
use style::CLASS;
use tw_macro::assert_component;

/// Top-level Collisions page: a breadcrumb bar above the two-pane content, filling
/// the view height so the content pane keeps its own scroll. Each kind renders a
/// sidebar (islands, hotkey units, or per-unit positions) beside a detail pane,
/// under a breadcrumb bar that swaps the active kind. Empty and all-clear states
/// replace the two-pane content when there is no file or no conflicts.
#[component]
pub fn CollisionsPage(props: CollisionsPageModel) -> Element {
    let model = use_collisions_page(&props);
    rsx! {
        div {
            class: CLASS,
            Breadcrumbs {
                breadcrumbs: model.breadcrumbs,
                aria_label: "Collision categories",
            }
            Body { content: model.content }
        }
    }
}

assert_component!(CollisionsPage);
