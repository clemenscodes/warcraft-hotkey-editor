use dioxus::prelude::*;

/// The published `View` contract mirroring [`PreviewTextareaProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PreviewTextareaView {
    pub text: ReadSignal<String>,
}

impl ddd::View for PreviewTextareaView {}
