use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

/// The published `View` contract mirroring [`PreviewDialogModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PreviewDialogView {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub preview_open: Signal<bool>,
}

impl ddd::View for PreviewDialogView {}
