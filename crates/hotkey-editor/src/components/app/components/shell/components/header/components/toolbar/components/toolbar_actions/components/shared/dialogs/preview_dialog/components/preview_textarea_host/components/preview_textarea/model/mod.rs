use super::view::PreviewTextareaView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PreviewTextareaModel {
    pub text: ReadSignal<String>,
}

impl From<&PreviewTextareaView> for PreviewTextareaModel {
    fn from(view: &PreviewTextareaView) -> Self {
        let PreviewTextareaView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for PreviewTextareaModel {
    type View = PreviewTextareaView;
}
