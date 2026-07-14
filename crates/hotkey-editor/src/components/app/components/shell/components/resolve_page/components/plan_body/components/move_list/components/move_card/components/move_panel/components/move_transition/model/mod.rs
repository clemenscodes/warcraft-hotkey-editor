use super::view::MoveTransitionView;
use crate::components::app::components::shell::components::resolve_page::presentation::MiniGridPlacement;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MoveTransitionModel {
    pub from_placements: Vec<MiniGridPlacement>,
    pub to_placements: Vec<MiniGridPlacement>,
}

impl From<&MoveTransitionView> for MoveTransitionModel {
    fn from(view: &MoveTransitionView) -> Self {
        let MoveTransitionView {
            from_placements,
            to_placements,
        } = view.clone();
        Self {
            from_placements,
            to_placements,
        }
    }
}

impl ddd::Model for MoveTransitionModel {
    type View = MoveTransitionView;
}
