#[derive(Clone, PartialEq)]
pub struct LosingCarrierBadgeView {
    pub count: usize,
}

impl ddd::Layered for LosingCarrierBadgeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for LosingCarrierBadgeView {}
