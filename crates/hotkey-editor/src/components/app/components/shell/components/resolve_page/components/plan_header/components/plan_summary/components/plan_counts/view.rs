/// The published `View` contract mirroring [`PlanCountsProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PlanCountsView {
    pub moves_text: String,
    pub unresolved_count: usize,
}

impl ddd::View for PlanCountsView {}
