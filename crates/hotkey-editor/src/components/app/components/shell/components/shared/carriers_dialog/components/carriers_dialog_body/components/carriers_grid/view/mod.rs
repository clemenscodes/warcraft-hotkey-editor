use crate::services::carriers::CarrierUnitView;

#[derive(Clone, PartialEq)]
pub struct CarriersGridView {
    pub carriers: Vec<CarrierUnitView>,
}

impl ddd::Layered for CarriersGridView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CarriersGridView {}
