use super::view::CarrierBadgeView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CarrierBadgeModel {
    pub count: usize,
    pub is_winner: bool,
}

impl From<&CarrierBadgeView> for CarrierBadgeModel {
    fn from(view: &CarrierBadgeView) -> Self {
        let CarrierBadgeView { count, is_winner } = view.clone();
        Self { count, is_winner }
    }
}

impl ddd::Layered for CarrierBadgeModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for CarrierBadgeModel {
    type View = CarrierBadgeView;
}
