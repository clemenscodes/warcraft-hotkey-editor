use super::view::CarriersGridView;
use crate::services::carriers::CarrierUnitView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CarriersGridProps {
    pub carriers: Vec<CarrierUnitView>,
}

impl From<&CarriersGridView> for CarriersGridProps {
    fn from(view: &CarriersGridView) -> Self {
        let CarriersGridView { carriers } = view.clone();
        Self { carriers }
    }
}

impl ddd::Props for CarriersGridProps {
    type View = CarriersGridView;
}
