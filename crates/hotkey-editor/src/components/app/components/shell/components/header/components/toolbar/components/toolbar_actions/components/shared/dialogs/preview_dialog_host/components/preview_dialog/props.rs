use super::view::PreviewDialogView;
use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

/// What the preview dialog needs: the loaded keys to serialize and the open
/// signal that drives the shell.
#[derive(Props, Clone, PartialEq)]
pub struct PreviewDialogProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub preview_open: Signal<bool>,
}

impl From<&PreviewDialogView> for PreviewDialogProps {
    fn from(view: &PreviewDialogView) -> Self {
        let PreviewDialogView {
            loaded_keys,
            preview_open,
        } = view.clone();
        Self {
            loaded_keys,
            preview_open,
        }
    }
}

impl ddd::Props for PreviewDialogProps {
    type View = PreviewDialogView;
}
