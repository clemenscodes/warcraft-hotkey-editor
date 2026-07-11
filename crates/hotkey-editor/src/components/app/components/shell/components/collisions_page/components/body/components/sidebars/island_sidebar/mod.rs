mod model;
mod presentation;
mod view;

pub use view::IslandSidebarView;

use super::shared::collision_sidebar::CollisionSidebar;
use crate::services::collision_selection::context::use_collision_selection;
use dioxus::prelude::*;
use model::IslandSidebarModel;
use presentation::cards;
use tw_macro::assert_component;

/// The island-collision sidebar: one card per collision island, handed to the
/// collision sidebar. The selected island is read from collision-selection context.
#[component]
pub fn IslandSidebar(props: IslandSidebarModel) -> Element {
    let selected_island = use_collision_selection().selected_island();
    let cards = cards(&props, selected_island);
    rsx! {
        CollisionSidebar { cards }
    }
}

assert_component!(IslandSidebar);
