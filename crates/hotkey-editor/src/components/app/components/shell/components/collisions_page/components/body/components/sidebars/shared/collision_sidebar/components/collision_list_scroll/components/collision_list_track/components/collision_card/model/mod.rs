use super::state::CollisionCardContent;
use super::view::CollisionCardView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionCardModel {
    pub is_selected: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub count: usize,
    pub content: CollisionCardContent,
}

impl From<&CollisionCardView> for CollisionCardModel {
    fn from(view: &CollisionCardView) -> Self {
        let CollisionCardView {
            is_selected,
            onclick,
            count,
            content,
        } = view.clone();
        Self {
            is_selected,
            onclick,
            count,
            content,
        }
    }
}

impl ddd::Model for CollisionCardModel {
    type View = CollisionCardView;
}
