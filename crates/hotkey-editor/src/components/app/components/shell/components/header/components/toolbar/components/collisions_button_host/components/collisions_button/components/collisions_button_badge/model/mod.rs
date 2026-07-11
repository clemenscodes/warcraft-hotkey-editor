use super::view::CollisionsButtonBadgeView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionsButtonBadgeModel {
    pub label: String,
}

impl From<&CollisionsButtonBadgeView> for CollisionsButtonBadgeModel {
    fn from(view: &CollisionsButtonBadgeView) -> Self {
        let CollisionsButtonBadgeView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Model for CollisionsButtonBadgeModel {
    type View = CollisionsButtonBadgeView;
}
