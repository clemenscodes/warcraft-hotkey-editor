use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;

/// The published `View` contract mirroring [`PositionsContentModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PositionsContentView {
    pub islands: Vec<IslandView>,
}

impl ddd::View for PositionsContentView {}
