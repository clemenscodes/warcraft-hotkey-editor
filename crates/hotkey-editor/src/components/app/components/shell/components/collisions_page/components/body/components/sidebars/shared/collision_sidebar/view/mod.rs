use super::components::collision_list_scroll::components::collision_list_track::components::collision_card::CollisionCardData;

#[derive(Clone, PartialEq)]
pub struct CollisionSidebarView {
    pub cards: Vec<CollisionCardData>,
}

impl ddd::View for CollisionSidebarView {}
