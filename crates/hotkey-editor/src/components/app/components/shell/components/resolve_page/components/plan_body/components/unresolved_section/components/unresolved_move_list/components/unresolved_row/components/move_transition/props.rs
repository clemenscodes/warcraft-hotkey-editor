use crate::components::app::components::shell::components::resolve_page::logic::MiniGridPlacement;
use dioxus::prelude::*;

/// The block flagging the cell the stuck ability lands on.
#[derive(Props, Clone, PartialEq)]
pub struct MoveTransitionProps {
    pub placements: Vec<MiniGridPlacement>,
}
