use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct UploadButtonInputView {
    pub on_change: EventHandler<FormEvent>,
}

impl ddd::View for UploadButtonInputView {}
