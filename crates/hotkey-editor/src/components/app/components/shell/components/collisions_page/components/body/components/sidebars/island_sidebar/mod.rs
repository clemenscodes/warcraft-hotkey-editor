mod logic;
mod props;

use super::shared::collision_sidebar::{CollisionSidebar, CollisionSidebarProps};
use dioxus::prelude::*;
use logic::cards;
pub use props::IslandSidebarProps;

/// The island-collision sidebar: one card per collision island, handed to the
/// collision sidebar.
use tw_macro::assert_component;
assert_component!(IslandSidebar);
#[component]
pub fn IslandSidebar(props: IslandSidebarProps) -> Element {
    let cards = cards(&props);
    let sidebar = CollisionSidebarProps { cards };
    rsx! {
        CollisionSidebar { ..sidebar }
    }
}
