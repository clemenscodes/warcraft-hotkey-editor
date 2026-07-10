mod logic;
mod props;

use super::shared::collision_sidebar::{CollisionSidebar, CollisionSidebarProps};
use crate::services::collision_selection::context::use_collision_selection;
use dioxus::prelude::*;
use logic::cards;
pub use props::IslandSidebarProps;
use tw_macro::assert_component;

/// The island-collision sidebar: one card per collision island, handed to the
/// collision sidebar. The selected island is read from collision-selection context.
#[component]
pub fn IslandSidebar(props: IslandSidebarProps) -> Element {
    let selected_island = use_collision_selection().selected_island();
    let cards = cards(&props, selected_island);
    let sidebar = CollisionSidebarProps { cards };
    rsx! {
        CollisionSidebar { ..sidebar }
    }
}

assert_component!(IslandSidebar);
