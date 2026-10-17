#[derive(Clone, PartialEq)]
pub struct GapPullReasonBadgeView {
    pub label: String,
}

impl ddd::Layered for GapPullReasonBadgeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for GapPullReasonBadgeView {}
