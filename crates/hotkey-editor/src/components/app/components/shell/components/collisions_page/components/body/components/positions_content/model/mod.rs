use super::view::PositionsContentView;
use crate::components::app::components::shell::components::collisions_page::presentation::IslandView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PositionsContentModel {
    pub islands: Vec<IslandView>,
}

impl From<&PositionsContentView> for PositionsContentModel {
    fn from(view: &PositionsContentView) -> Self {
        let PositionsContentView { islands } = view.clone();
        Self { islands }
    }
}

impl ddd::Model for PositionsContentModel {
    type View = PositionsContentView;
}
