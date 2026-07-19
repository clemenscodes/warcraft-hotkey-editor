use super::view::CollisionPagerView;
use crate::components::app::components::shell::components::collisions_page::components::body::ContentModel;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionPagerModel {
    pub content: ContentModel,
}

impl From<&CollisionPagerView> for CollisionPagerModel {
    fn from(view: &CollisionPagerView) -> Self {
        let CollisionPagerView { content } = view.clone();
        Self { content }
    }
}

impl ddd::Model for CollisionPagerModel {
    type View = CollisionPagerView;
}
