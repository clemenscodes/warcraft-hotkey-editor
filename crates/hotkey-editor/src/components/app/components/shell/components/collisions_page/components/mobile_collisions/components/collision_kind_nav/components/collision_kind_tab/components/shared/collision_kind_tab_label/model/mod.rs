use super::view::CollisionKindTabLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionKindTabLabelModel {
    #[props(into)]
    pub text: String,
}

impl From<&CollisionKindTabLabelView> for CollisionKindTabLabelModel {
    fn from(view: &CollisionKindTabLabelView) -> Self {
        let CollisionKindTabLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for CollisionKindTabLabelModel {
    type View = CollisionKindTabLabelView;
}
