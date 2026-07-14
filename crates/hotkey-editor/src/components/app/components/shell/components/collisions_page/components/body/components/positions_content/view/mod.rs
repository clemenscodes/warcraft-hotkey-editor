use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;

#[derive(Clone, PartialEq)]
pub struct PositionsContentView {
    pub islands: Vec<IslandView>,
}

impl ddd::View for PositionsContentView {}
