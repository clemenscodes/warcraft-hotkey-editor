#[derive(Clone, PartialEq)]
pub struct PlanSummaryView {
    pub moves_text: String,
    pub unresolved_count: usize,
}

impl ddd::View for PlanSummaryView {}
