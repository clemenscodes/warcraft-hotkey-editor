#[derive(Clone, PartialEq)]
pub struct PlanCountsView {
    pub moves_text: String,
    pub unresolved_count: usize,
}

impl ddd::View for PlanCountsView {}
