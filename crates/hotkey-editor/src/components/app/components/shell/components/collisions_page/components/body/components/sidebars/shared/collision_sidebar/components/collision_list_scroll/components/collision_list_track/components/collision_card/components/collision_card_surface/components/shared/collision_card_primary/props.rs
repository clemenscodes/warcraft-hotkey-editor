use super::view::CollisionCardPrimaryView;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;
use dioxus::prelude::*;

/// The primary meta line of a collision card, chosen by the card's content: the
/// unit's name and object id, or the island's highlighted coordinate.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionCardPrimaryProps {
    pub content: CollisionCardContent,
}

impl From<&CollisionCardPrimaryView> for CollisionCardPrimaryProps {
    fn from(view: &CollisionCardPrimaryView) -> Self {
        let CollisionCardPrimaryView { content } = view.clone();
        Self { content }
    }
}

impl ddd::Props for CollisionCardPrimaryProps {
    type View = CollisionCardPrimaryView;
}
