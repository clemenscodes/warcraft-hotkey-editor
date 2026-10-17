#[derive(Clone, PartialEq)]
pub struct FilterTriggerBadgeView {
    pub count: usize,
}

impl ddd::Layered for FilterTriggerBadgeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for FilterTriggerBadgeView {}
