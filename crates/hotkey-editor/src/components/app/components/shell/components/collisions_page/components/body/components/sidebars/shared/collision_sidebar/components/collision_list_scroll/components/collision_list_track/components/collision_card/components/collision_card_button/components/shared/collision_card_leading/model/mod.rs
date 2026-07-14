use super::view::CollisionCardLeadingView;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::state::CollisionCardContent;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionCardLeadingModel {
    pub content: CollisionCardContent,
}

impl From<&CollisionCardLeadingView> for CollisionCardLeadingModel {
    fn from(view: &CollisionCardLeadingView) -> Self {
        let CollisionCardLeadingView { content } = view.clone();
        Self { content }
    }
}

impl ddd::Model for CollisionCardLeadingModel {
    type View = CollisionCardLeadingView;
}
