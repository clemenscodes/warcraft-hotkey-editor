#[derive(Clone, PartialEq)]
pub struct StuckReasonBadgeView {
    pub label: String,
}

impl ddd::Layered for StuckReasonBadgeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for StuckReasonBadgeView {}
