use super::components::collision_list_track::components::collision_card::CollisionCardData;

/// The published `View` contract mirroring [`CollisionListScrollProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CollisionListScrollView {
    pub cards: Vec<CollisionCardData>,
}

impl ddd::View for CollisionListScrollView {}
