#[derive(Clone, PartialEq)]
pub struct FightReasonBadgeView {
    pub label: String,
}

impl ddd::Layered for FightReasonBadgeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for FightReasonBadgeView {}
