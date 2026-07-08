mod logic;
mod props;

use super::shared::collision_sidebar::{CollisionSidebar, CollisionSidebarProps};
use dioxus::prelude::*;
use logic::cards;
pub use props::UnitCardsSidebarProps;

/// The unit-collision sidebar: one card per clashing unit, handed to the collision
/// sidebar. Generic over the conflict shape, so the hotkey and unit-position kinds
/// render this one sidebar.
use tw_macro::assert_component;
assert_component!(UnitCardsSidebar);
#[component]
pub fn UnitCardsSidebar<Conflict: Clone + PartialEq + 'static>(
    props: UnitCardsSidebarProps<Conflict>,
) -> Element {
    let cards = cards(&props);
    let sidebar = CollisionSidebarProps { cards };
    rsx! {
        CollisionSidebar { ..sidebar }
    }
}
