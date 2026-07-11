use dioxus::prelude::*;

/// The published `View` contract mirroring [`PrimaryButtonModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PrimaryButtonView {
    pub onclick: EventHandler<MouseEvent>,
    pub label: String,
}

impl ddd::View for PrimaryButtonView {}
