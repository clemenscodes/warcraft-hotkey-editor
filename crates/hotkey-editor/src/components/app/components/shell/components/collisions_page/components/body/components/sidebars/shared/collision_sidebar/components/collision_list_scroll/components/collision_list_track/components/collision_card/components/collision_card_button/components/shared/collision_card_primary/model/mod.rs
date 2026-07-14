use super::view::CollisionCardPrimaryView;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionCardPrimaryModel {
    pub content: CollisionCardContent,
}

impl From<&CollisionCardPrimaryView> for CollisionCardPrimaryModel {
    fn from(view: &CollisionCardPrimaryView) -> Self {
        let CollisionCardPrimaryView { content } = view.clone();
        Self { content }
    }
}

impl ddd::Model for CollisionCardPrimaryModel {
    type View = CollisionCardPrimaryView;
}
