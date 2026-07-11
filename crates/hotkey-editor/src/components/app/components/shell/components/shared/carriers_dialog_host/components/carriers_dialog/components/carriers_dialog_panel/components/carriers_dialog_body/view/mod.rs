use crate::services::carriers::CarrierUnitView;

/// The published `View` contract mirroring [`CarriersDialogBodyModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CarriersDialogBodyView {
    pub carriers: Vec<CarrierUnitView>,
}

impl ddd::View for CarriersDialogBodyView {}
