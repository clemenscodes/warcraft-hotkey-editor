use crate::services::carriers::CarrierUnitView;

/// The published `View` contract mirroring [`CarriersGridProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CarriersGridView {
    pub carriers: Vec<CarrierUnitView>,
}

impl ddd::View for CarriersGridView {}
