use dioxus::prelude::*;

/// The published `View` contract mirroring [`SecondaryButtonProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SecondaryButtonView {
    pub onclick: EventHandler<MouseEvent>,
    pub label: String,
}

impl ddd::View for SecondaryButtonView {}
