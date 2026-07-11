use dioxus::prelude::*;
use warcraft_keybinds::CollisionSummary;

/// The published `View` contract mirroring [`CollisionsButtonProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CollisionsButtonView {
    pub summary: CollisionSummary,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for CollisionsButtonView {}
