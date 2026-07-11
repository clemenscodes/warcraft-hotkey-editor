use std::rc::Rc;
use warcraft_keybinds::GridSlotId;
use warcraft_keybinds::InspectorDetail;

/// The published `View` contract mirroring [`TileOverrideModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TileOverrideView {
    pub detail: InspectorDetail,
    pub active_container_slots: Rc<[GridSlotId]>,
}

impl ddd::View for TileOverrideView {}
