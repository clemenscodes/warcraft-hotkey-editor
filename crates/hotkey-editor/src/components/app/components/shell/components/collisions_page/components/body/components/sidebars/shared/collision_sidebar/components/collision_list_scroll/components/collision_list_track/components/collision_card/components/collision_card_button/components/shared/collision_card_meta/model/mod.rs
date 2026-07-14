use super::view::CollisionCardMetaView;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionCardMetaModel {
    pub content: CollisionCardContent,
    pub count: usize,
}

impl From<&CollisionCardMetaView> for CollisionCardMetaModel {
    fn from(view: &CollisionCardMetaView) -> Self {
        let CollisionCardMetaView { content, count } = view.clone();
        Self { content, count }
    }
}

impl ddd::Model for CollisionCardMetaModel {
    type View = CollisionCardMetaView;
}
