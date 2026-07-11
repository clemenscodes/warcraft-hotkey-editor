use super::view::CarrierCardView;
use crate::services::carriers::CarrierUnitView;
use dioxus::prelude::*;

/// One carrier of an ability in the carriers dialog: the carrier view it renders as an
/// icon, name, and id that deep-link into the editor focused on that unit. The navigation
/// used to open the unit is read from context, so it is not a prop.
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
