use dioxus::prelude::*;

/// The "N collisions" count line on a collision card.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionCountProps {
    pub count: usize,
}
