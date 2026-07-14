use super::view::MiniGridView;
use crate::components::app::components::shell::components::resolve_page::presentation::MiniGridPlacement;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MiniGridModel {
    pub placements: Vec<MiniGridPlacement>,
}

impl From<&MiniGridView> for MiniGridModel {
    fn from(view: &MiniGridView) -> Self {
        let MiniGridView { placements } = view.clone();
        Self { placements }
    }
}

impl ddd::Model for MiniGridModel {
    type View = MiniGridView;
}
