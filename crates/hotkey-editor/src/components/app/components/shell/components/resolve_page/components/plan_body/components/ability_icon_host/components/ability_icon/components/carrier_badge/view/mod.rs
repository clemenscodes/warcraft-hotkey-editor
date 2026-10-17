#[derive(Clone, PartialEq)]
pub struct CarrierBadgeView {
    pub count: usize,
    pub is_winner: bool,
}

impl ddd::Layered for CarrierBadgeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CarrierBadgeView {}
