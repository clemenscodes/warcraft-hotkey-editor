use dioxus::prelude::*;

/// The published `View` contract mirroring [`PreviewDialogBodyProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PreviewDialogBodyView {
    pub text: ReadSignal<String>,
}

impl ddd::View for PreviewDialogBodyView {}
