use super::components::collision_card::CollisionCardData;
use dioxus::prelude::*;

/// The track lays out the collision cards it is handed.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionListTrackProps {
    pub cards: Vec<CollisionCardData>,
}
