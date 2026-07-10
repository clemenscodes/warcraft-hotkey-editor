use crate::components::app::components::shell::components::collisions_page::logic::IslandView;

/// The published `View` contract mirroring [`IslandDetailProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IslandDetailView {
    pub islands: Vec<IslandView>,
}

impl ddd::View for IslandDetailView {}
