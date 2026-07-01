mod logic;
mod props;
mod style;

use super::collision_card::CollisionCard;
use crate::assert_component;
use crate::components::unit_list::components::unit_list_scroll::UnitListScroll;
use dioxus::prelude::*;
use logic::cards;
pub use props::UnitPositionSidebarProps;
use style::CLASS;
assert_component!(UnitPositionSidebar);

/// A scrollable list of clashing-unit cards.
#[component]
pub fn UnitPositionSidebar(props: UnitPositionSidebarProps) -> Element {
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
