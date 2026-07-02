mod logic;
mod props;
mod style;

use super::collision_card::CollisionCard;
use crate::assert_component;
use crate::components::unit_list::components::unit_list_scroll::UnitListScroll;
use dioxus::prelude::*;
use logic::cards;
pub use props::IslandSidebarProps;
use style::CLASS;
assert_component!(IslandSidebar);

/// The island sidebar: a scrollable list of collision-island cards.
#[component]
pub fn IslandSidebar(props: IslandSidebarProps) -> Element {
    let cards = cards(&props);
    rsx! {
        aside {
            class: CLASS,
            UnitListScroll {
                for card in cards {
                    CollisionCard { ..card }
                }
            }
        }
    }
}
