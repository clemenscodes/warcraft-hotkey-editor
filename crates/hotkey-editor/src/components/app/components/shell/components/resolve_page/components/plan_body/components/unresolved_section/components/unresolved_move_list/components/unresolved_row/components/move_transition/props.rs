use super::view::MoveTransitionView;
use crate::components::app::components::shell::components::resolve_page::logic::MiniGridPlacement;
use dioxus::prelude::*;

/// The block flagging the cell the stuck ability lands on.
#[derive(Props, Clone, PartialEq)]
pub struct MoveTransitionProps {
    pub placements: Vec<MiniGridPlacement>,
}

impl From<&MoveTransitionView> for MoveTransitionProps {
    fn from(view: &MoveTransitionView) -> Self {
        let MoveTransitionView { placements } = view.clone();
        Self { placements }
    }
}

impl ddd::Props for MoveTransitionProps {
    type View = MoveTransitionView;
}
