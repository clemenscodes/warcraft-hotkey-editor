#[derive(Clone, PartialEq)]
pub struct SpillReasonBadgeView {
    pub label: String,
}

impl ddd::Layered for SpillReasonBadgeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SpillReasonBadgeView {}
