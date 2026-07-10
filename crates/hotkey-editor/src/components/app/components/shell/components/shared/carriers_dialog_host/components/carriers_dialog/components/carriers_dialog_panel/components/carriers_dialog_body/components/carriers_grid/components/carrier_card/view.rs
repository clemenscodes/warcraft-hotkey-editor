use crate::services::carriers::CarrierUnitView;

/// The published `View` contract mirroring [`CarrierCardProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CarrierCardView {
    pub carrier: CarrierUnitView,
}

impl ddd::View for CarrierCardView {}
