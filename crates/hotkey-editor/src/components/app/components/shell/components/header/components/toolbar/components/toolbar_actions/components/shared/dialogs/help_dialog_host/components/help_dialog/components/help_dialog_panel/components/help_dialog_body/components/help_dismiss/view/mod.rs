use dioxus::prelude::*;

/// The published `View` contract mirroring [`HelpDismissModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpDismissView {
    pub on_dismiss: EventHandler<MouseEvent>,
}

impl ddd::View for HelpDismissView {}
