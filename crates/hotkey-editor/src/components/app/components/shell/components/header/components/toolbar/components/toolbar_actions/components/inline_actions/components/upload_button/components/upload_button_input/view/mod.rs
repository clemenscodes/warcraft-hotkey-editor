use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct UploadButtonInputView {
    pub on_change: EventHandler<FormEvent>,
}

impl ddd::Layered for UploadButtonInputView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UploadButtonInputView {}
