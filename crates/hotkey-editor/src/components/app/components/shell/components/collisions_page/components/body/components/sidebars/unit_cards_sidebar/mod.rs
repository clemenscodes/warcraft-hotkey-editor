mod logic;
mod props;

use super::shared::collision_card::CollisionCard;
use super::sidebar::components::collision_list_scroll::CollisionScroll;
use super::sidebar::{Sidebar, SidebarProps};
use dioxus::prelude::*;
use logic::cards;
pub use props::UnitCardsSidebarProps;

/// The unit-collision sidebar: one card per clashing unit, fed into the base
/// sidebar bound to the collision scroll. Generic over the conflict shape, so the
/// hotkey and unit-position kinds render this one sidebar.
use tw_macro::assert_component;
assert_component!(UnitCardsSidebar);
#[component]
pub fn UnitCardsSidebar<Conflict: Clone + PartialEq + 'static>(
    props: UnitCardsSidebarProps<Conflict>,
) -> Element {
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
