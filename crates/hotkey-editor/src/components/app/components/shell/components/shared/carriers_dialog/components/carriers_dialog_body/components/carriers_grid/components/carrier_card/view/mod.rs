use crate::services::carriers::CarrierUnitView;

#[derive(Clone, PartialEq)]
pub struct CarrierCardView {
    pub carrier: CarrierUnitView,
}

impl ddd::View for CarrierCardView {}
