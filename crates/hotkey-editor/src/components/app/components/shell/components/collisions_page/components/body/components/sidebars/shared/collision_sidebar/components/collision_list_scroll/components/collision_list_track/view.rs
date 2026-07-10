use super::components::collision_card::CollisionCardData;

/// The published `View` contract mirroring [`CollisionListTrackProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CollisionListTrackView {
    pub cards: Vec<CollisionCardData>,
}

impl ddd::View for CollisionListTrackView {}
