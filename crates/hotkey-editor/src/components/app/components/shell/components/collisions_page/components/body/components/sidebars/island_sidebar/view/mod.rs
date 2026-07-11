use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;

/// The published `View` contract mirroring [`IslandSidebarModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IslandSidebarView {
    pub islands: Vec<IslandView>,
}

impl ddd::View for IslandSidebarView {}
