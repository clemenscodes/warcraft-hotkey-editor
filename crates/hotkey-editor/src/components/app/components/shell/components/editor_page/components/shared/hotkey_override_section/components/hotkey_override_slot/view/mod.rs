use std::rc::Rc;
use warcraft_keybinds::{GridSlotId, InspectorDetail};

#[derive(Clone, PartialEq)]
pub struct HotkeyOverrideSlotView {
    pub detail: Option<InspectorDetail>,
    pub active_container_slots: Rc<[GridSlotId]>,
}

impl ddd::View for HotkeyOverrideSlotView {}
