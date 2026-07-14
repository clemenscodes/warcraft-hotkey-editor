use super::view::SystemHotkeysListEntryView;
use dioxus::prelude::*;
use warcraft_keybinds::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysListEntryModel {
    pub section_id: WarcraftObjectId,
    pub comment: String,
}

impl From<&SystemHotkeysListEntryView> for SystemHotkeysListEntryModel {
    fn from(view: &SystemHotkeysListEntryView) -> Self {
        let SystemHotkeysListEntryView {
            section_id,
            comment,
        } = view.clone();
        Self {
            section_id,
            comment,
        }
    }
}

impl ddd::Model for SystemHotkeysListEntryModel {
    type View = SystemHotkeysListEntryView;
}
