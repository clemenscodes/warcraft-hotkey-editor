use crate::services::carriers::CarrierUnitView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CarriersGridProps {
    pub carriers: Vec<CarrierUnitView>,
}
