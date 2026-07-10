use dioxus::prelude::*;

/// The published `View` contract mirroring [`SystemHotkeysDialogProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SystemHotkeysDialogView {
    pub system_hotkeys_open: Signal<bool>,
}

impl ddd::View for SystemHotkeysDialogView {}
