use std::rc::Rc;
use warcraft_keybinds::{GridSlotId, InspectorDetail};

/// The published `View` contract mirroring [`UnitTileOverrideProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitTileOverrideView {
    pub detail: Option<InspectorDetail>,
    pub active_container_slots: Rc<[GridSlotId]>,
}

impl ddd::View for UnitTileOverrideView {}
