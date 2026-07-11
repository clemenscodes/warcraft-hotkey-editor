use super::view::UploadButtonInputView;
use dioxus::prelude::*;

/// The hidden file input forwards its change event to the upload button's handler.
#[derive(Props, Clone, PartialEq)]
pub struct UploadButtonInputModel {
    pub on_change: EventHandler<FormEvent>,
}

impl From<&UploadButtonInputView> for UploadButtonInputModel {
    fn from(view: &UploadButtonInputView) -> Self {
        let UploadButtonInputView { on_change } = view.clone();
        Self { on_change }
    }
}

impl ddd::Model for UploadButtonInputModel {
    type View = UploadButtonInputView;
}
