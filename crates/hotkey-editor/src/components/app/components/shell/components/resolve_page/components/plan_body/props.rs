use super::components::active_move_list::components::move_row::MoveRowProps;
use super::components::unresolved_section::components::unresolved_row::UnresolvedRowProps;
use dioxus::prelude::*;

/// The active section's move cards, tagged with its category slug.
#[derive(Clone, PartialEq)]
pub struct PlanBodySection {
    data_category: &'static str,
    rows: Vec<MoveRowProps>,
}

impl PlanBodySection {
    pub fn new(data_category: &'static str, rows: Vec<MoveRowProps>) -> Self {
        Self {
            data_category,
            rows,
        }
    }

    pub fn data_category(&self) -> &'static str {
        self.data_category
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
