use crate::components::app::components::shell::components::collisions_page::logic::IslandView;

/// The published `View` contract mirroring [`IslandSidebarProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IslandSidebarView {
    pub islands: Vec<IslandView>,
}

impl ddd::View for IslandSidebarView {}
