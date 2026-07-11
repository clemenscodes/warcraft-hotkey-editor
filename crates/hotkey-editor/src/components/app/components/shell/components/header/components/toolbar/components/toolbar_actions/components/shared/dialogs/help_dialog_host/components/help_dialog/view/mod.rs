use dioxus::prelude::*;

/// The published `View` contract mirroring [`HelpDialogModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpDialogView {
    pub help_open: Signal<bool>,
}

impl ddd::View for HelpDialogView {}
