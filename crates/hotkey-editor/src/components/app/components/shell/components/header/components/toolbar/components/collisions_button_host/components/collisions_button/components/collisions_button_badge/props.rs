use super::view::CollisionsButtonBadgeView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionsButtonBadgeProps {
    pub label: String,
}

impl From<&CollisionsButtonBadgeView> for CollisionsButtonBadgeProps {
    fn from(view: &CollisionsButtonBadgeView) -> Self {
        let CollisionsButtonBadgeView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Props for CollisionsButtonBadgeProps {
    type View = CollisionsButtonBadgeView;
}
