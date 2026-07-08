use super::super::super::state::CollisionCardContent;
use dioxus::prelude::*;

/// The leading visual of a collision card, chosen by the card's content: a unit
/// portrait or an island mini grid.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionCardVisualProps {
    pub content: CollisionCardContent,
}
