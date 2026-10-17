#[derive(Clone, PartialEq)]
pub struct CollisionsButtonBadgeView {
    pub label: String,
}

impl ddd::Layered for CollisionsButtonBadgeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CollisionsButtonBadgeView {}
