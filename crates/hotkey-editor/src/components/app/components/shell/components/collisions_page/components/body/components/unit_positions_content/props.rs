use crate::components::app::components::shell::components::collisions_page::components::body::components::details::unit_position_detail::UnitPositionDetailProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::unit_cards_sidebar::UnitCardsSidebarProps;
use crate::components::app::components::shell::components::collisions_page::logic::UnitPositionConflictView;
use crate::services::navigation::app_view::CollisionKind;
use dioxus::prelude::*;

/// The per-unit position-collision two-pane content: the clashing-units sidebar beside
/// the unit position detail pane, tagged with the fixed `unit-positions` kind slug and
/// the conflict count for the e2e hooks.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionsContentProps {
    pub count: usize,
    pub sidebar: UnitCardsSidebarProps<UnitPositionConflictView>,
    pub detail: UnitPositionDetailProps,
}

/// The kind slug and count resolved for the two-pane wrapper's data attributes.
pub(super) struct UnitPositionsContentPresentation {
    pub collision_kind: &'static str,
    pub count: usize,
}

impl From<&UnitPositionsContentProps> for UnitPositionsContentPresentation {
    fn from(props: &UnitPositionsContentProps) -> Self {
        let kind = CollisionKind::UnitPositions;
        let collision_kind = kind.kind_param();
        let count = props.count;
        Self {
            collision_kind,
            count,
        }
    }
}
