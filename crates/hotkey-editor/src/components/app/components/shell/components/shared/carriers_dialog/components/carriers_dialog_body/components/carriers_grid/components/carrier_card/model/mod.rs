use super::view::CarrierCardView;
use crate::services::carriers::CarrierUnitView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CarrierCardModel {
    pub carrier: CarrierUnitView,
}

impl From<&CarrierCardView> for CarrierCardModel {
    fn from(view: &CarrierCardView) -> Self {
        let CarrierCardView { carrier } = view.clone();
        Self { carrier }
    }
}

impl ddd::Model for CarrierCardModel {
    type View = CarrierCardView;
}
