use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;

/// The published `View` contract mirroring [`FilledIslandDetailModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FilledIslandDetailView {
    pub island: IslandView,
}

impl ddd::View for FilledIslandDetailView {}
