use super::view::MoveTransitionView;
use crate::components::app::components::shell::components::resolve_page::presentation::MiniGridPlacement;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MoveTransitionModel {
    pub placements: Vec<MiniGridPlacement>,
}

impl From<&MoveTransitionView> for MoveTransitionModel {
    fn from(view: &MoveTransitionView) -> Self {
        let MoveTransitionView { placements } = view.clone();
        Self { placements }
    }
}

impl ddd::Model for MoveTransitionModel {
    type View = MoveTransitionView;
}
