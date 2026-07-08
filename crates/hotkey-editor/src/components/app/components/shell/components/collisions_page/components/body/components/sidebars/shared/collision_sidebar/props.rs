use super::components::collision_list_scroll::components::collision_list_track::components::collision_card::CollisionCardProps;
use dioxus::prelude::*;

/// The collision sidebar's props: the collision cards it lays out in its scroll
/// region.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionSidebarProps {
    pub cards: Vec<CollisionCardProps>,
}
