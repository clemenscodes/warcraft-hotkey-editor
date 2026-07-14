use super::components::collision_list_track::components::collision_card::CollisionCardData;
use super::view::CollisionListScrollView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionListScrollModel {
    pub cards: Vec<CollisionCardData>,
}

impl From<&CollisionListScrollView> for CollisionListScrollModel {
    fn from(view: &CollisionListScrollView) -> Self {
        let CollisionListScrollView { cards } = view.clone();
        Self { cards }
    }
}

impl ddd::Model for CollisionListScrollModel {
    type View = CollisionListScrollView;
}
