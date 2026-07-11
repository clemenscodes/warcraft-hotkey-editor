use super::state::CollisionCardContent;
use super::view::CollisionCardView;
use dioxus::prelude::*;

/// A selectable collision-sidebar card: its selected state, click handler, live
/// collision count, and the content that fills it (a unit portrait with name/id, or
/// an island's highlighted coordinate).
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
