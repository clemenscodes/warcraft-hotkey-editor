use super::HotkeyUpgradePositionPickerBody;
use super::model::HotkeyUpgradePositionPickerBodyModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

#[derive(Clone, PartialEq, Default)]
pub struct HotkeyUpgradePositionPickerBodyView {
    pub upgrade_unit_id: WarcraftObjectId,
    pub picker_slots: Rc<[GridSlotId]>,
}

impl ddd::View for HotkeyUpgradePositionPickerBodyView {}

impl Render for HotkeyUpgradePositionPickerBodyView {
    type Model = HotkeyUpgradePositionPickerBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let upgrade_unit_id = self.upgrade_unit_id;
        let picker_slots = self.picker_slots.clone();
        rsx! {
            HotkeyUpgradePositionPickerBody {
                upgrade_unit_id,
                picker_slots,
            }
        }
    }
}
