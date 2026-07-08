use super::super::super::state::CollisionCardContent;
use dioxus::prelude::*;

/// The primary meta line of a collision card, chosen by the card's content: the
/// unit's name and object id, or the island's highlighted coordinate.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionCardPrimaryProps {
    pub content: CollisionCardContent,
}
