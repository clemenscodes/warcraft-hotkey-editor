use dioxus::prelude::*;

/// The published `View` contract mirroring [`InfoActionsProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct InfoActionsView {
    pub primary_label: &'static str,
    pub on_primary: EventHandler<MouseEvent>,
    pub on_cancel: EventHandler<MouseEvent>,
}

impl ddd::View for InfoActionsView {}
