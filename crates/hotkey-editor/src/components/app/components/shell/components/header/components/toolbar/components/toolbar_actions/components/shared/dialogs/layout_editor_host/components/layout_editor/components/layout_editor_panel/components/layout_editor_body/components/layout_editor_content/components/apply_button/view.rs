use dioxus::prelude::*;

/// The published `View` contract mirroring [`ApplyButtonProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ApplyButtonView {
    pub on_apply: EventHandler<MouseEvent>,
}

impl ddd::View for ApplyButtonView {}
