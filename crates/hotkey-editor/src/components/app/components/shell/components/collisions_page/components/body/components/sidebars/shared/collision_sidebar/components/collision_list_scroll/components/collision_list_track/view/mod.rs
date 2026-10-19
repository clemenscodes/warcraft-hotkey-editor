use super::components::collision_card::CollisionCardData;

#[derive(Clone, PartialEq)]
pub struct CollisionListTrackView {
    pub cards: Vec<CollisionCardData>,
}

impl ddd::Layered for CollisionListTrackView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CollisionListTrackView {}
