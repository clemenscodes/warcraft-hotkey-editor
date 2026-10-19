#[derive(Clone, PartialEq)]
pub struct PlanSummaryView {
    pub moves_text: String,
    pub unresolved_count: usize,
}

impl ddd::Layered for PlanSummaryView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PlanSummaryView {}
