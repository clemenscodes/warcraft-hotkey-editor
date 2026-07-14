use super::view::CollisionSidebarView;
use super::components::collision_list_scroll::components::collision_list_track::components::collision_card::CollisionCardData;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionSidebarModel {
    pub cards: Vec<CollisionCardData>,
}

impl From<&CollisionSidebarView> for CollisionSidebarModel {
    fn from(view: &CollisionSidebarView) -> Self {
        let CollisionSidebarView { cards } = view.clone();
        Self { cards }
    }
}

impl ddd::Model for CollisionSidebarModel {
    type View = CollisionSidebarView;
}
