mod model;
mod presentation;
mod view;
pub use view::UnitCardsSidebarView;

use super::shared::collision_sidebar::CollisionSidebar;
use dioxus::prelude::*;
use model::UnitCardsSidebarModel;
use presentation::{
    SelectedCollisionUnit, UnitCardsSidebarPresentation, use_unit_cards_sidebar_presentation,
};
use tw_macro::assert_component;

#[component]
pub fn UnitCardsSidebar<Conflict: Clone + PartialEq + SelectedCollisionUnit + 'static>(
    props: UnitCardsSidebarModel<Conflict>,
) -> Element {
    let UnitCardsSidebarPresentation { cards, .. } = use_unit_cards_sidebar_presentation(&props);
    rsx! {
        CollisionSidebar {
            cards,
        }
    }
}

assert_component!(UnitCardsSidebar);
