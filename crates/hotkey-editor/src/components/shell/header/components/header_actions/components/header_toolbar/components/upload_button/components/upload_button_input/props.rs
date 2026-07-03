use dioxus::prelude::*;

/// The hidden file input forwards its change event to the upload button's handler.
#[derive(Props, Clone, PartialEq)]
pub struct UploadButtonInputProps {
    pub on_change: EventHandler<FormEvent>,
}
