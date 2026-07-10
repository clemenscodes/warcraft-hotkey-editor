use dioxus::prelude::*;

/// The published `View` contract mirroring [`UploadButtonInputProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UploadButtonInputView {
    pub on_change: EventHandler<FormEvent>,
}

impl ddd::View for UploadButtonInputView {}
