use super::components::collision_card::CollisionCardData;
use super::view::CollisionListTrackView;
use dioxus::prelude::*;

/// The track lays out the collision cards it is handed.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionListTrackModel {
    pub cards: Vec<CollisionCardData>,
}

impl From<&CollisionListTrackView> for CollisionListTrackModel {
    fn from(view: &CollisionListTrackView) -> Self {
        let CollisionListTrackView { cards } = view.clone();
        Self { cards }
    }
}

impl ddd::Model for CollisionListTrackModel {
    type View = CollisionListTrackView;
}
