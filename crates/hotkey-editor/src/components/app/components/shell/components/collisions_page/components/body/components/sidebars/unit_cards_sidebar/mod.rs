mod logic;
mod props;

use super::shared::collision_sidebar::CollisionSidebar;
use crate::services::collision_selection::context::use_collision_selection;
use dioxus::prelude::*;
use logic::{SelectedCollisionUnit, cards};
use props::UnitCardsSidebarProps;
use tw_macro::assert_component;

/// The unit-collision sidebar: one card per clashing unit, handed to the collision
/// sidebar. Generic over the conflict shape, so the hotkey and unit-position kinds
/// render this one sidebar. The selected unit is read from collision-selection context
/// (the conflict kind names which field), mirroring `IslandSidebar`.
#[component]
pub fn UnitCardsSidebar<Conflict: Clone + PartialEq + SelectedCollisionUnit + 'static>(
    props: UnitCardsSidebarProps<Conflict>,
) -> Element {
    let collision_selection = use_collision_selection();
    let selected_unit = Conflict::selected_unit(collision_selection);
    let cards = cards(&props, selected_unit);
    rsx! {
        CollisionSidebar { cards }
    }
}

assert_component!(UnitCardsSidebar);
