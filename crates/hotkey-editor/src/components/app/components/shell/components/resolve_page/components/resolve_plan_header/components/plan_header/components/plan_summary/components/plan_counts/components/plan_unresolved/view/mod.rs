/// The published `View` contract mirroring [`PlanUnresolvedModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PlanUnresolvedView {
    pub count: usize,
}

impl ddd::View for PlanUnresolvedView {}
