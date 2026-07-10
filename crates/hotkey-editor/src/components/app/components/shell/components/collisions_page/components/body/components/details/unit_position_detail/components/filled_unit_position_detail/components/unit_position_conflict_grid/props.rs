use crate::components::app::components::shell::components::collisions_page::logic::UnitPositionConflictView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The scrolling grid of position-collision cards for the selected unit: one card per
/// conflict, all deep-linking through the owning unit id.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionConflictGridProps {
    pub conflicts: Vec<UnitPositionConflictView>,
    pub unit_id: WarcraftObjectId,
}
