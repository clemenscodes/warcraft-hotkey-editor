use super::state::ButtonVariant;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`ButtonProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ButtonView {
    pub variant: ButtonVariant,
    pub onclick: EventHandler<MouseEvent>,
    pub label: String,
}

impl ddd::View for ButtonView {}
