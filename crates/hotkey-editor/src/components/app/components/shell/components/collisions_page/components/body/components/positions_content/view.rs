use crate::components::app::components::shell::components::collisions_page::logic::IslandView;

/// The published `View` contract mirroring [`PositionsContentProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PositionsContentView {
    pub islands: Vec<IslandView>,
}

impl ddd::View for PositionsContentView {}
