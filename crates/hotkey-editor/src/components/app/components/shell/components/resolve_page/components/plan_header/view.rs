use dioxus::prelude::*;

/// The published `View` contract mirroring [`PlanHeaderProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PlanHeaderView {
    pub moves_text: String,
    pub unresolved_count: usize,
    pub running: bool,
    pub on_apply: EventHandler<MouseEvent>,
}

impl ddd::View for PlanHeaderView {}
