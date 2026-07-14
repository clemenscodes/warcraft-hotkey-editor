use super::view::LosingCarrierBadgeView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LosingCarrierBadgeModel {
    pub count: usize,
}

impl From<&LosingCarrierBadgeView> for LosingCarrierBadgeModel {
    fn from(view: &LosingCarrierBadgeView) -> Self {
        let LosingCarrierBadgeView { count } = view.clone();
        Self { count }
    }
}

impl ddd::Model for LosingCarrierBadgeModel {
    type View = LosingCarrierBadgeView;
}
