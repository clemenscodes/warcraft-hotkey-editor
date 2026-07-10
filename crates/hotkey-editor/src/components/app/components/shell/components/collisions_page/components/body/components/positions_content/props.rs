use super::view::PositionsContentView;
use crate::components::app::components::shell::components::collisions_page::logic::IslandView;
use dioxus::prelude::*;

/// The position-collision two-pane content: the collision islands the sidebar and the
/// detail pane both render.
#[derive(Props, Clone, PartialEq)]
pub struct PositionsContentProps {
    pub islands: Vec<IslandView>,
}

impl From<&PositionsContentView> for PositionsContentProps {
    fn from(view: &PositionsContentView) -> Self {
        let PositionsContentView { islands } = view.clone();
        Self { islands }
    }
}

impl ddd::Props for PositionsContentProps {
    type View = PositionsContentView;
}
