#[derive(Clone, PartialEq)]
pub struct WinnerCarrierBadgeView {
    pub count: usize,
}

impl ddd::Layered for WinnerCarrierBadgeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for WinnerCarrierBadgeView {}
