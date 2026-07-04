use dioxus::prelude::*;

/// The textarea's only input: the serialized text to show. A read-only reactive
/// handle so the textarea re-renders when the loaded keys change.
#[derive(Props, Clone, PartialEq)]
pub struct PreviewTextareaProps {
    pub text: ReadSignal<String>,
}
