use std::rc::Rc;
use warcraft_keybinds::{GridSlotId, InspectorDetail};

/// The published `View` contract mirroring [`HotkeyOverrideSlotModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HotkeyOverrideSlotView {
    pub detail: Option<InspectorDetail>,
    pub active_container_slots: Rc<[GridSlotId]>,
}

impl ddd::View for HotkeyOverrideSlotView {}
