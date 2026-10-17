#[derive(Clone, PartialEq)]
pub struct PlanCountsView {
    pub moves_text: String,
    pub unresolved_count: usize,
}

impl ddd::Layered for PlanCountsView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PlanCountsView {}
