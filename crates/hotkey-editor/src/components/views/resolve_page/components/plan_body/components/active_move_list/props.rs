use crate::components::views::resolve_page::components::plan_body::PlanBodySection;
use dioxus::prelude::*;

/// The active category's move section to render, or nothing when the plan has only
/// unresolved abilities and no moves to show.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveMoveListProps {
    pub section: Option<PlanBodySection>,
}
