use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_keybinds::{GridSlotId, InspectorDetail};

/// The override slot's inputs: the inspector detail (absent when no tile is
/// selected) and the container slots its override card edits against. The editor
/// signals the card drives are sourced from context by the card's own hook.
#[derive(Props, Clone, PartialEq)]
pub struct UnitTileOverrideProps {
    pub detail: Option<InspectorDetail>,
    pub active_container_slots: Rc<[GridSlotId]>,
}
