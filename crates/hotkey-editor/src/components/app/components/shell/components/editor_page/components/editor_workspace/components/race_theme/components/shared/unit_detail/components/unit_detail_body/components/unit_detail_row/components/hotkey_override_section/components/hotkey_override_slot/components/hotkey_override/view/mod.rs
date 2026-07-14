use std::rc::Rc;
use warcraft_keybinds::GridSlotId;
use warcraft_keybinds::InspectorDetail;

#[derive(Clone, PartialEq)]
pub struct HotkeyOverrideView {
    pub detail: InspectorDetail,
    pub active_container_slots: Rc<[GridSlotId]>,
}

impl ddd::View for HotkeyOverrideView {}
