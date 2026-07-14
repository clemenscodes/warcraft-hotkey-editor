use super::HotkeyAltPositionPickerDialogBody;
use super::model::HotkeyAltPositionPickerDialogBodyModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

/// The published `View` contract for the off-state position picker's dialog body region.
/// It carries only the picker's plain, signal-free identity — the object being edited and
/// its slot set — so it satisfies the `Render` region's `Default` bound; the live grid
/// editor config (a signal bundle) is assembled inside the body's own presentation, never
/// threaded through this contract. It is also the dialog's body region: it `impl Render`
/// and renders `HotkeyAltPositionPickerDialogBody` once, so the host places the published
/// `View` directly as `WarcraftDialog`'s body, with no ad-hoc region type.
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
