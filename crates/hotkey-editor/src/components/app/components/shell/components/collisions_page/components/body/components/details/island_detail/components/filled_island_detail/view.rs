use crate::components::app::components::shell::components::collisions_page::logic::IslandView;

/// The published `View` contract mirroring [`FilledIslandDetailProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FilledIslandDetailView {
    pub island: IslandView,
}

impl ddd::View for FilledIslandDetailView {}
