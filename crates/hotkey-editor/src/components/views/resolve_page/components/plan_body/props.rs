use super::components::active_move_list::components::move_row::MoveRowProps;
use super::components::unresolved_section::components::unresolved_row::UnresolvedRowProps;
use dioxus::prelude::*;

/// The active section's move cards, tagged with its category slug.
#[derive(Clone, PartialEq)]
pub struct PlanBodySection {
    pub data_category: &'static str,
    pub rows: Vec<MoveRowProps>,
}

/// The scrollable plan body: the active move section and the unresolved section.
#[derive(Props, Clone, PartialEq)]
pub struct PlanBodyProps {
    pub section: Option<PlanBodySection>,
    pub unresolved_rows: Vec<UnresolvedRowProps>,
}
