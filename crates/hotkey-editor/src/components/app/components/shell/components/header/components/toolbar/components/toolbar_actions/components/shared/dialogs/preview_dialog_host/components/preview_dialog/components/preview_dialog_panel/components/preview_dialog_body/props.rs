use super::components::preview_textarea::PreviewTextareaProps;
use dioxus::prelude::*;

/// The preview dialog's scroll region input: the serialized-text textarea it holds.
#[derive(Props, Clone, PartialEq)]
pub struct PreviewDialogBodyProps {
    pub textarea: PreviewTextareaProps,
}

impl From<&PreviewDialogBodyProps> for PreviewTextareaProps {
    fn from(props: &PreviewDialogBodyProps) -> Self {
        props.textarea.clone()
    }
}
