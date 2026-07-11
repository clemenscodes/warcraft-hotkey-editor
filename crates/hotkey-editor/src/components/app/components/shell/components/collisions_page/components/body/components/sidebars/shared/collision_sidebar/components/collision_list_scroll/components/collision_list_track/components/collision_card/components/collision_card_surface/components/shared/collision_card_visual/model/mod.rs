use super::view::CollisionCardVisualView;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;
use dioxus::prelude::*;

/// The leading visual of a collision card, chosen by the card's content: a unit
/// portrait or an island mini grid.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionCardVisualModel {
    pub content: CollisionCardContent,
}

impl From<&CollisionCardVisualView> for CollisionCardVisualModel {
    fn from(view: &CollisionCardVisualView) -> Self {
        let CollisionCardVisualView { content } = view.clone();
        Self { content }
    }
}

impl ddd::Model for CollisionCardVisualModel {
    type View = CollisionCardVisualView;
}
