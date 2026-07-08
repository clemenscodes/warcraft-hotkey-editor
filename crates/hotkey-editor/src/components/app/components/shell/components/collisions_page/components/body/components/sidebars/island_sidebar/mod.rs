mod logic;
mod props;

use super::shared::collision_card::CollisionCard;
use super::sidebar::components::collision_list_scroll::CollisionScroll;
use super::sidebar::{Sidebar, SidebarProps};
use dioxus::prelude::*;
use logic::cards;
pub use props::IslandSidebarProps;

/// The island-collision sidebar: one card per collision island, fed into the base
/// sidebar bound to the collision scroll.
use tw_macro::assert_component;
assert_component!(IslandSidebar);
#[component]
pub fn IslandSidebar(props: IslandSidebarProps) -> Element {
    let cards = cards(&props);
    let children = rsx! {
        for card in cards {
            CollisionCard { ..card }
        }
    };
    let kind = CollisionScroll;
    let sidebar = SidebarProps { kind, children };
    rsx! {
        Sidebar { ..sidebar }
    }
}
