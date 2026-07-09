use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::active_move_list::components::move_row::components::move_panel::MovePanelProps;
use crate::components::app::components::shell::components::resolve_page::logic::MiniGridPlacement;
use dioxus::prelude::*;

/// The from → to transition block below the abilities: the placements for the "before"
/// and "after" mini grids.
#[derive(Props, Clone, PartialEq)]
pub struct MoveTransitionProps {
    pub from_placements: Vec<MiniGridPlacement>,
    pub to_placements: Vec<MiniGridPlacement>,
}

impl From<&MovePanelProps> for MoveTransitionProps {
    fn from(props: &MovePanelProps) -> Self {
        let from_placements = props.from_placements.clone();
        let to_placements = props.to_placements.clone();
        Self {
            from_placements,
            to_placements,
        }
    }
}
