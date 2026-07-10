use super::components::collision_card::CollisionCardData;
use super::view::CollisionListTrackView;
use dioxus::prelude::*;

/// The track lays out the collision cards it is handed.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionListTrackProps {
    pub cards: Vec<CollisionCardData>,
}

impl From<&CollisionListTrackView> for CollisionListTrackProps {
    fn from(view: &CollisionListTrackView) -> Self {
        let CollisionListTrackView { cards } = view.clone();
        Self { cards }
    }
}

impl ddd::Props for CollisionListTrackProps {
    type View = CollisionListTrackView;
}
