use super::super::super::super::super::state::CollisionCardContent;
use dioxus::prelude::*;

/// The text column of a collision card: the primary meta line (name and id, or the
/// coordinate) above the collision count.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionCardMetaProps {
    pub content: CollisionCardContent,
    pub count: usize,
}
