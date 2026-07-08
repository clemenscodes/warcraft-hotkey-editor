use super::components::collision_list_track::components::collision_card::CollisionCardProps;
use dioxus::prelude::*;

/// The collision list scroll carries the collision cards down to its track.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionListScrollProps {
    pub cards: Vec<CollisionCardProps>,
}
