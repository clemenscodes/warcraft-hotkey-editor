use crate::components::app::components::shell::components::collisions_page::components::body::components::details::island_detail::IslandDetailProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::island_sidebar::IslandSidebarProps;
use dioxus::prelude::*;

/// The position-collision two-pane content: the island sidebar beside the island
/// detail pane.
#[derive(Props, Clone, PartialEq)]
pub struct PositionsContentProps {
    pub sidebar: IslandSidebarProps,
    pub detail: IslandDetailProps,
}
