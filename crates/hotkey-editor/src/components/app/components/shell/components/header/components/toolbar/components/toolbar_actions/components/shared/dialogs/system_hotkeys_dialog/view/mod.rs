use dioxus::prelude::*;

/// The published `View` contract mirroring [`SystemHotkeysDialogModel`](super::model::SystemHotkeysDialogModel),
/// threaded to this component as data: the open value it drives and the change handler that
/// mirrors the headless dialog's own close back to the trigger that owns the open signal.
#[derive(Clone, PartialEq)]
pub struct SystemHotkeysDialogView {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl ddd::View for SystemHotkeysDialogView {}
