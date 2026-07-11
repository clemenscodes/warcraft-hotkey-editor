use super::state::CollisionCardContent;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`CollisionCardModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CollisionCardView {
    pub is_selected: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub count: usize,
    pub content: CollisionCardContent,
}

impl ddd::View for CollisionCardView {}
