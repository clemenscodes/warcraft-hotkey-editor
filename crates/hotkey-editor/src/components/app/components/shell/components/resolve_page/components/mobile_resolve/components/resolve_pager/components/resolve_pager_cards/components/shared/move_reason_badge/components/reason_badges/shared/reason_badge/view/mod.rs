#[derive(Clone, PartialEq)]
pub struct ReasonBadgeView {
    pub label: String,
}

impl ddd::Layered for ReasonBadgeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ReasonBadgeView {}
