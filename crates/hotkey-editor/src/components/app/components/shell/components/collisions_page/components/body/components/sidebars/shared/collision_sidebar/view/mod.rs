use super::components::collision_list_scroll::components::collision_list_track::components::collision_card::CollisionCardData;

/// The published `View` contract mirroring [`CollisionSidebarModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CollisionSidebarView {
    pub cards: Vec<CollisionCardData>,
}

impl ddd::View for CollisionSidebarView {}
