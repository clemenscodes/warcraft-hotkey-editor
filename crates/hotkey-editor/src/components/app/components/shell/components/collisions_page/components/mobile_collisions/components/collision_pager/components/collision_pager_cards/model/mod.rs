use super::view::CollisionPagerCardsView;
use crate::components::app::components::shell::components::collisions_page::components::body::ContentModel;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionPagerCardsModel {
    pub content: ContentModel,
}

impl From<&CollisionPagerCardsView> for CollisionPagerCardsModel {
    fn from(view: &CollisionPagerCardsView) -> Self {
        let CollisionPagerCardsView { content } = view.clone();
        Self { content }
    }
}

impl ddd::Model for CollisionPagerCardsModel {
    type View = CollisionPagerCardsView;
}
