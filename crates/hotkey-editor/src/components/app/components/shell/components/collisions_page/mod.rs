pub mod components;
mod data;
mod frame;
mod model;
pub mod presentation;
mod view;

pub use view::CollisionsPageView;
mod style;

use components::body::BodyView;
use components::collisions_breadcrumbs::CollisionsBreadcrumbsView;
use dioxus::prelude::*;
use dioxus_kit::frame::Page;
use frame::CollisionsPageFrame;
use model::CollisionsPageModel;
use presentation::use_collisions_page;
use style::CLASS;
use tw_macro::assert_component;

/// Top-level Collisions page: a breadcrumb bar above the two-pane content, filling
/// the view height so the content pane keeps its own scroll. Each kind renders a
/// sidebar (islands, hotkey units, or per-unit positions) beside a detail pane,
/// under a breadcrumb bar that swaps the active kind. Empty and all-clear states
/// replace the two-pane content when there is no file or no conflicts. It composes the
/// headless `Page` frame from its header (the breadcrumb bar) and body (the two-pane
/// content) regions, styling the container with its own `CLASS`.
#[component]
pub fn CollisionsPage(props: CollisionsPageModel) -> Element {
    let model = use_collisions_page(&props);
    let breadcrumbs = model.breadcrumbs;
    let content = model.content;
    let header = CollisionsBreadcrumbsView { breadcrumbs };
    let body = BodyView { content };
    let frame = CollisionsPageFrame { header, body };
    rsx! {
        Page {
            class: CLASS,
            frame,
        }
    }
}

assert_component!(CollisionsPage);
