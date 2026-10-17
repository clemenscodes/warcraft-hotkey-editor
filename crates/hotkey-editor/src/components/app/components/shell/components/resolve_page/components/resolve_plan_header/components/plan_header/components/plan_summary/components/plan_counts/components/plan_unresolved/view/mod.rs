#[derive(Clone, PartialEq)]
pub struct PlanUnresolvedView {
    pub count: usize,
}

impl ddd::Layered for PlanUnresolvedView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PlanUnresolvedView {}
