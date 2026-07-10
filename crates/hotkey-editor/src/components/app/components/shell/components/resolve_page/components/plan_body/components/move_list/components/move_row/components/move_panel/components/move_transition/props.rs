use crate::components::app::components::shell::components::resolve_page::logic::MiniGridPlacement;
use dioxus::prelude::*;

/// The from → to transition block below the abilities: the placements for the "before"
/// and "after" mini grids.
#[derive(Props, Clone, PartialEq)]
pub struct MoveTransitionProps {
    pub from_placements: Vec<MiniGridPlacement>,
    pub to_placements: Vec<MiniGridPlacement>,
}
