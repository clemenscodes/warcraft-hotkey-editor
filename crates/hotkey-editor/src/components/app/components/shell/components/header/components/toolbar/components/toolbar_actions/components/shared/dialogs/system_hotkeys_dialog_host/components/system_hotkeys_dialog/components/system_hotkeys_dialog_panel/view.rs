use dioxus::prelude::*;

/// The published `View` contract mirroring [`SystemHotkeysDialogPanelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SystemHotkeysDialogPanelView {
    pub title: String,
    pub on_close: EventHandler<()>,
}

impl ddd::View for SystemHotkeysDialogPanelView {}
