use super::view::CarriersGridView;
use crate::services::carriers::CarrierUnitView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CarriersGridModel {
    pub carriers: Vec<CarrierUnitView>,
}

impl From<&CarriersGridView> for CarriersGridModel {
    fn from(view: &CarriersGridView) -> Self {
        let CarriersGridView { carriers } = view.clone();
        Self { carriers }
    }
}

impl ddd::Model for CarriersGridModel {
    type View = CarriersGridView;
}
