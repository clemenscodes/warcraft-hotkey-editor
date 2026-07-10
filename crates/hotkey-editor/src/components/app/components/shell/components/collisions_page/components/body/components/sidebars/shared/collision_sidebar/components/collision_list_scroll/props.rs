use super::components::collision_list_track::components::collision_card::CollisionCardData;
use super::view::CollisionListScrollView;
use dioxus::prelude::*;

/// The collision list scroll carries the collision cards down to its track.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionListScrollProps {
    pub cards: Vec<CollisionCardData>,
}

impl From<&CollisionListScrollView> for CollisionListScrollProps {
    fn from(view: &CollisionListScrollView) -> Self {
        let CollisionListScrollView { cards } = view.clone();
        Self { cards }
    }
}

impl ddd::Props for CollisionListScrollProps {
    type View = CollisionListScrollView;
}
