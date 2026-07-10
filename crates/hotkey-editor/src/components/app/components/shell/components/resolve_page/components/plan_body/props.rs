use super::components::move_list::components::move_row::MoveRowProps;
use super::components::unresolved_section::components::unresolved_move_list::components::unresolved_row::UnresolvedRowProps;
use dioxus::prelude::*;

/// The active section's move cards.
#[derive(Clone, PartialEq)]
pub struct PlanBodySection {
    rows: Vec<MoveRowProps>,
}

impl PlanBodySection {
    pub fn new(rows: Vec<MoveRowProps>) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &[MoveRowProps] {
        &self.rows
    }
}

/// The scrollable plan body: the active move section and the unresolved section.
#[derive(Props, Clone, PartialEq)]
pub struct PlanBodyProps {
    pub section: Option<PlanBodySection>,
    pub unresolved_rows: Vec<UnresolvedRowProps>,
}
