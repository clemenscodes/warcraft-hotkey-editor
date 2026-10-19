use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PreviewTextareaView {
    pub text: ReadSignal<String>,
}

impl ddd::Layered for PreviewTextareaView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PreviewTextareaView {}
