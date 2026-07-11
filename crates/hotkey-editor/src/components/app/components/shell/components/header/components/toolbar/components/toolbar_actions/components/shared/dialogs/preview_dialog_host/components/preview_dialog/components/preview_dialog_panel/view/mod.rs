use dioxus::prelude::*;

/// The published `View` contract mirroring [`PreviewDialogPanelModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PreviewDialogPanelView {
    pub title: String,
    pub on_close: EventHandler<()>,
    pub text: ReadSignal<String>,
}

impl ddd::View for PreviewDialogPanelView {}
