use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_keybinds::GridSlotId;
use warcraft_keybinds::InspectorDetail;

/// The per-tile override editor: the panel that edits the selected ability's hotkey,
/// off-state, upgraded form, and command-card position. The shared editor signals it
/// drives are sourced from context by the component's hook; only the inspected detail
/// and its container slots are props.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideProps {
    pub detail: InspectorDetail,
    pub active_container_slots: Rc<[GridSlotId]>,
}
