use dioxus::prelude::*;

/// The published `View` contract mirroring [`SystemHotkeysDialogModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SystemHotkeysDialogView {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl ddd::View for SystemHotkeysDialogView {}
