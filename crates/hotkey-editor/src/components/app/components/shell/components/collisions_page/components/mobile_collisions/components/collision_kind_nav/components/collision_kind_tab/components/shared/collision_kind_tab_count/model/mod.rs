use super::view::CollisionKindTabCountView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionKindTabCountModel {
    pub count: usize,
}

impl From<&CollisionKindTabCountView> for CollisionKindTabCountModel {
    fn from(view: &CollisionKindTabCountView) -> Self {
        let CollisionKindTabCountView { count } = view.clone();
        Self { count }
    }
}

impl ddd::Model for CollisionKindTabCountModel {
    type View = CollisionKindTabCountView;
}
