mod logic;
mod props;

use super::shared::unit_card::UnitCard;
use super::sidebar::components::collision_list_scroll::CollisionScroll;
use super::sidebar::{Sidebar, SidebarProps};
use dioxus::prelude::*;
use logic::cards;
pub use props::HotkeyUnitSidebarProps;

/// The hotkey-collision sidebar: one card per clashing unit, fed into the base
/// sidebar bound to the collision scroll.
#[component]
pub fn HotkeyUnitSidebar(props: HotkeyUnitSidebarProps) -> Element {
    let cards = cards(&props);
    let children = rsx! {
        for card in cards {
            UnitCard { ..card }
        }
    };
    let kind = CollisionScroll;
    let sidebar = SidebarProps { kind, children };
    rsx! {
        Sidebar { ..sidebar }
    }
}
