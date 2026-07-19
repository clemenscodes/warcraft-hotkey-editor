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
use components::mobile_collisions::MobileCollisions;
use dioxus::prelude::*;
use dioxus_kit::frame::Page;
use frame::CollisionsPageFrame;
use model::CollisionsPageModel;
use crate::services::viewport::use_is_touch_viewport;
use presentation::use_collisions_page;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CollisionsPage(props: CollisionsPageModel) -> Element {
    let model = use_collisions_page(&props);
    let breadcrumbs = model.breadcrumbs;
    let content = model.content;
    let is_touch = use_is_touch_viewport();
    if is_touch {
        return rsx! {
            MobileCollisions {
                breadcrumbs,
                content,
            }
        };
    }
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
