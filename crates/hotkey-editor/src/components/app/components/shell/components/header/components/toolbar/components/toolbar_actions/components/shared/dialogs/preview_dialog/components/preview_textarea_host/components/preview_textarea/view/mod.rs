use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PreviewTextareaView {
    pub text: ReadSignal<String>,
}

impl ddd::View for PreviewTextareaView {}
