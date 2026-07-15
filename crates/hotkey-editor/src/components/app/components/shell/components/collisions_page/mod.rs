pub mod components;
mod data;
mod model;
pub mod presentation;
mod view;

pub use view::CollisionsPageView;
mod style;

use super::shared::warcraft_page::WarcraftPage;
use components::body::BodyView;
use components::collisions_breadcrumbs::CollisionsBreadcrumbsView;
use dioxus::prelude::*;
use model::CollisionsPageModel;
use presentation::use_collisions_page;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CollisionsPage(props: CollisionsPageModel) -> Element {
    let model = use_collisions_page(&props);
    let breadcrumbs = model.breadcrumbs;
    let content = model.content;
    let header = CollisionsBreadcrumbsView { breadcrumbs };
    let body = BodyView { content };
    rsx! {
        div {
            class: CLASS,
            WarcraftPage::<CollisionsBreadcrumbsView, BodyView> {
                header,
                body,
            }
        }
    }
}

assert_component!(CollisionsPage);
