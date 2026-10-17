#[derive(Clone, PartialEq)]
pub struct SwapReasonBadgeView {
    pub label: String,
}

impl ddd::Layered for SwapReasonBadgeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SwapReasonBadgeView {}
