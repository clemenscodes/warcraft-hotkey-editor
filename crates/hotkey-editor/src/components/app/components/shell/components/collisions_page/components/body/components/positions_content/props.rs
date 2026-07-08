use crate::components::app::components::shell::components::collisions_page::components::body::components::details::island_detail::IslandDetailProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::island_sidebar::IslandSidebarProps;
use crate::services::navigation::app_view::CollisionKind;
use dioxus::prelude::*;

/// The position-collision two-pane content: the island sidebar beside the island
/// detail pane, tagged with the fixed `positions` kind slug and the conflict count
/// for the e2e hooks.
#[derive(Props, Clone, PartialEq)]
pub struct PositionsContentProps {
    pub count: usize,
    pub sidebar: IslandSidebarProps,
    pub detail: IslandDetailProps,
}

/// The kind slug and count resolved for the two-pane wrapper's data attributes.
pub(super) struct PositionsContentPresentation {
    pub collision_kind: &'static str,
    pub count: usize,
}

impl From<&PositionsContentProps> for PositionsContentPresentation {
    fn from(props: &PositionsContentProps) -> Self {
        let kind = CollisionKind::Positions;
        let collision_kind = kind.kind_param();
        let count = props.count;
        Self {
            collision_kind,
            count,
        }
    }
}
