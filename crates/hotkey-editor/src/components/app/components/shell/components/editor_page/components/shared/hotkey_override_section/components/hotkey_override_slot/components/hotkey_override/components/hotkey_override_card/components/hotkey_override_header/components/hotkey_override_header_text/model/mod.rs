use super::view::HotkeyOverrideHeaderTextView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyOverrideHeaderTextModel {
    pub name_text: String,
    pub object_id: WarcraftObjectId,
}

impl From<&HotkeyOverrideHeaderTextView> for HotkeyOverrideHeaderTextModel {
    fn from(view: &HotkeyOverrideHeaderTextView) -> Self {
        let HotkeyOverrideHeaderTextView {
            name_text,
            object_id,
        } = view.clone();
        Self {
            name_text,
            object_id,
        }
    }
}

impl ddd::Model for HotkeyOverrideHeaderTextModel {
    type View = HotkeyOverrideHeaderTextView;
}
