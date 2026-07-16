use super::HotkeyAltPositionPickerDialogBody;
use super::model::HotkeyAltPositionPickerDialogBodyModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

#[derive(Clone, PartialEq, Default)]
pub struct HotkeyAltPositionPickerDialogBodyView {
    pub object_id: WarcraftObjectId,
    pub picker_slots: Rc<[GridSlotId]>,
}

impl ddd::View for HotkeyAltPositionPickerDialogBodyView {}

impl Render for HotkeyAltPositionPickerDialogBodyView {
    type Model = HotkeyAltPositionPickerDialogBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let object_id = self.object_id;
        let picker_slots = self.picker_slots.clone();
        rsx! {
            HotkeyAltPositionPickerDialogBody {
                object_id,
                picker_slots,
            }
        }
    }
}
