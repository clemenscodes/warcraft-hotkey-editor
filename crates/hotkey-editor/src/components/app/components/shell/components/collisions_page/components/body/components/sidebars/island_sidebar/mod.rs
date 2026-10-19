mod model;
mod presentation;
mod view;

pub use view::IslandSidebarView;

use super::shared::collision_sidebar::CollisionSidebar;
use dioxus::prelude::*;
use model::IslandSidebarModel;
use presentation::{IslandSidebarPresentation, use_island_sidebar_presentation};
use tw_macro::assert_component;

#[component]
pub fn IslandSidebar(props: IslandSidebarModel) -> Element {
    let IslandSidebarPresentation { cards } = use_island_sidebar_presentation(&props);
    rsx! {
        CollisionSidebar {
            cards,
        }
    }
}

assert_component!(IslandSidebar);
