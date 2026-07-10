use super::view::PreviewTextareaView;
use dioxus::prelude::*;

/// The textarea's only input: the serialized text to show. A read-only reactive
/// handle so the textarea re-renders when the loaded keys change.
#[derive(Props, Clone, PartialEq)]
pub struct PreviewTextareaProps {
    pub text: ReadSignal<String>,
}

impl From<&PreviewTextareaView> for PreviewTextareaProps {
    fn from(view: &PreviewTextareaView) -> Self {
        let PreviewTextareaView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for PreviewTextareaProps {
    type View = PreviewTextareaView;
}
