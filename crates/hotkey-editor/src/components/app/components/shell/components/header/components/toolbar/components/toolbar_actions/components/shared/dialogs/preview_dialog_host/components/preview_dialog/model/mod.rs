use super::view::PreviewDialogView;
use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

/// What the preview dialog needs: the loaded keys to serialize and the open
/// signal that drives the shell.
#[derive(Props, Clone, PartialEq)]
pub struct PreviewDialogModel {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub preview_open: Signal<bool>,
}

impl From<&PreviewDialogView> for PreviewDialogModel {
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

impl ddd::Model for PreviewDialogModel {
    type View = PreviewDialogView;
}
