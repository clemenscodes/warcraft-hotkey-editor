use super::view::CollisionCountView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionCountModel {
    pub count: usize,
}

impl From<&CollisionCountView> for CollisionCountModel {
    fn from(view: &CollisionCountView) -> Self {
        let CollisionCountView { count } = view.clone();
        Self { count }
    }
}

impl ddd::Model for CollisionCountModel {
    type View = CollisionCountView;
}
