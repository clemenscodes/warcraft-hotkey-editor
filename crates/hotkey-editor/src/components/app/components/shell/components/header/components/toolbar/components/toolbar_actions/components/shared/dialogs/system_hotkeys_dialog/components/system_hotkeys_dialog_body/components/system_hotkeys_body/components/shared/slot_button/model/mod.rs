use super::view::SlotButtonView;
use dioxus::prelude::*;
use warcraft_keybinds::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct SlotButtonModel {
    pub slot_label: String,
    pub section_id: WarcraftObjectId,
}

impl From<&SlotButtonView> for SlotButtonModel {
    fn from(view: &SlotButtonView) -> Self {
        let SlotButtonView {
            slot_label,
            section_id,
        } = view.clone();
        Self {
            slot_label,
            section_id,
        }
    }
}

impl ddd::Model for SlotButtonModel {
    type View = SlotButtonView;
}
