use crate::services::carriers::CarrierUnitView;

#[derive(Clone, PartialEq)]
pub struct CarrierCardView {
    pub carrier: CarrierUnitView,
}

impl ddd::Layered for CarrierCardView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CarrierCardView {}
