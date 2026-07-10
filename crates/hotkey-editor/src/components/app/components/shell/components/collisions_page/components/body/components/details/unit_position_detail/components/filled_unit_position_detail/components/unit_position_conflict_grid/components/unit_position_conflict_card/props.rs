use super::view::UnitPositionConflictCardView;
use crate::components::app::components::shell::components::collisions_page::logic::UnitPositionConflictView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// One position conflict on a unit: the clashing abilities and the owning unit id the
/// ability icons deep-link through. The icons read the navigation from context, so no
/// navigation is threaded through this card.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionConflictCardProps {
    pub conflict: UnitPositionConflictView,
    pub unit_id: WarcraftObjectId,
}

impl From<&UnitPositionConflictCardView> for UnitPositionConflictCardProps {
    fn from(view: &UnitPositionConflictCardView) -> Self {
        let UnitPositionConflictCardView { conflict, unit_id } = view.clone();
        Self { conflict, unit_id }
    }
}

impl ddd::Props for UnitPositionConflictCardProps {
    type View = UnitPositionConflictCardView;
}
