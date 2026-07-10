use super::view::CollisionCardMetaView;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;
use dioxus::prelude::*;

/// The text column of a collision card: the primary meta line (name and id, or the
/// coordinate) above the collision count.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionCardMetaProps {
    pub content: CollisionCardContent,
    pub count: usize,
}

impl From<&CollisionCardMetaView> for CollisionCardMetaProps {
    fn from(view: &CollisionCardMetaView) -> Self {
        let CollisionCardMetaView { content, count } = view.clone();
        Self { content, count }
    }
}

impl ddd::Props for CollisionCardMetaProps {
    type View = CollisionCardMetaView;
}
