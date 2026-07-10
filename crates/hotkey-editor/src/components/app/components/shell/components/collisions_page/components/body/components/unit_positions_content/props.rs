use crate::components::app::components::shell::components::collisions_page::components::body::components::details::unit_position_detail::UnitPositionDetailProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::unit_cards_sidebar::UnitCardsSidebarProps;
use crate::components::app::components::shell::components::collisions_page::logic::UnitPositionConflictView;
use dioxus::prelude::*;

/// The per-unit position-collision two-pane content: the clashing-units sidebar beside
/// the unit position detail pane.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionsContentProps {
    pub sidebar: UnitCardsSidebarProps<UnitPositionConflictView>,
    pub detail: UnitPositionDetailProps,
}
