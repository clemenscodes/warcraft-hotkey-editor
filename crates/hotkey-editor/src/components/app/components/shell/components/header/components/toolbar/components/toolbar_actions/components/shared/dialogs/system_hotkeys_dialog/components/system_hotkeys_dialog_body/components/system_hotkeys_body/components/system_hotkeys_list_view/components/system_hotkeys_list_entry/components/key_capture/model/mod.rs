use super::view::KeyCaptureView;
use dioxus::prelude::*;
use warcraft_keybinds::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct KeyCaptureModel {
    pub section_id: WarcraftObjectId,
}

impl From<&KeyCaptureView> for KeyCaptureModel {
    fn from(view: &KeyCaptureView) -> Self {
        let KeyCaptureView { section_id } = view.clone();
        Self { section_id }
    }
}

impl ddd::Model for KeyCaptureModel {
    type View = KeyCaptureView;
}
