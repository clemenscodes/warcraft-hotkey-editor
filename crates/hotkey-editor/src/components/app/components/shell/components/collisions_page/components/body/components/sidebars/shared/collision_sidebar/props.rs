use super::view::CollisionSidebarView;
use super::components::collision_list_scroll::components::collision_list_track::components::collision_card::CollisionCardData;
use dioxus::prelude::*;

/// The collision sidebar's props: the collision cards it lays out in its scroll
/// region.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionSidebarProps {
    pub cards: Vec<CollisionCardData>,
}

impl From<&CollisionSidebarView> for CollisionSidebarProps {
    fn from(view: &CollisionSidebarView) -> Self {
        let CollisionSidebarView { cards } = view.clone();
        Self { cards }
    }
}

impl ddd::Props for CollisionSidebarProps {
    type View = CollisionSidebarView;
}
