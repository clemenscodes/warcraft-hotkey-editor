use super::view::HotkeyOverrideSlotView;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_keybinds::{GridSlotId, InspectorDetail};

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyOverrideSlotModel {
    pub detail: Option<InspectorDetail>,
    pub active_container_slots: Rc<[GridSlotId]>,
}

impl From<&HotkeyOverrideSlotView> for HotkeyOverrideSlotModel {
    fn from(view: &HotkeyOverrideSlotView) -> Self {
        let HotkeyOverrideSlotView {
            detail,
            active_container_slots,
        } = view.clone();
        Self {
            detail,
            active_container_slots,
        }
    }
}

impl ddd::Model for HotkeyOverrideSlotModel {
    type View = HotkeyOverrideSlotView;
}
