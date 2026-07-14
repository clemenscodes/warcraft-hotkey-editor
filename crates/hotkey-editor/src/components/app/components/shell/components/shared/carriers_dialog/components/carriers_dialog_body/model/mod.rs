use super::view::CarriersDialogBodyView;
use crate::services::carriers::CarrierUnitView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CarriersDialogBodyModel {
    pub carriers: Vec<CarrierUnitView>,
}

impl From<&CarriersDialogBodyView> for CarriersDialogBodyModel {
    fn from(view: &CarriersDialogBodyView) -> Self {
        let CarriersDialogBodyView { carriers } = view.clone();
        Self { carriers }
    }
}

impl ddd::Model for CarriersDialogBodyModel {
    type View = CarriersDialogBodyView;
}
