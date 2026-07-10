use super::view::CollisionCountView;
use dioxus::prelude::*;

/// The "N collisions" count line on a collision card.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionCountProps {
    pub count: usize,
}

impl From<&CollisionCountView> for CollisionCountProps {
    fn from(view: &CollisionCountView) -> Self {
        let CollisionCountView { count } = view.clone();
        Self { count }
    }
}

impl ddd::Props for CollisionCountProps {
    type View = CollisionCountView;
}
