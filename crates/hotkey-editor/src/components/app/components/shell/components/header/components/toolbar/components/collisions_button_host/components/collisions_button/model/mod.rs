use super::view::CollisionsButtonView;
use dioxus::prelude::*;
use warcraft_keybinds::CollisionSummary;

#[derive(Props, Clone, PartialEq)]
pub struct CollisionsButtonModel {
    pub summary: CollisionSummary,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&CollisionsButtonView> for CollisionsButtonModel {
    fn from(view: &CollisionsButtonView) -> Self {
        let CollisionsButtonView { summary, onclick } = view.clone();
        Self { summary, onclick }
    }
}

impl ddd::Model for CollisionsButtonModel {
    type View = CollisionsButtonView;
}
