use dioxus::prelude::*;
use warcraft_keybinds::CollisionSummary;

#[derive(Clone, PartialEq)]
pub struct CollisionsButtonView {
    pub summary: CollisionSummary,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for CollisionsButtonView {}
