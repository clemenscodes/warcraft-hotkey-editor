use super::view::UploadButtonInputView;
use dioxus::prelude::*;

/// The hidden file input forwards its change event to the upload button's handler.
#[derive(Props, Clone, PartialEq)]
pub struct UploadButtonInputProps {
    pub on_change: EventHandler<FormEvent>,
}

impl From<&UploadButtonInputView> for UploadButtonInputProps {
    fn from(view: &UploadButtonInputView) -> Self {
        let UploadButtonInputView { on_change } = view.clone();
        Self { on_change }
    }
}

impl ddd::Props for UploadButtonInputProps {
    type View = UploadButtonInputView;
}
