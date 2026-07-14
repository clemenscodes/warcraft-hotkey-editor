#[derive(Clone, PartialEq)]
pub struct PlanUnresolvedView {
    pub count: usize,
}

impl ddd::View for PlanUnresolvedView {}
