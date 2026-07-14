use super::view::WinnerCarrierBadgeView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WinnerCarrierBadgeModel {
    pub count: usize,
}

impl From<&WinnerCarrierBadgeView> for WinnerCarrierBadgeModel {
    fn from(view: &WinnerCarrierBadgeView) -> Self {
        let WinnerCarrierBadgeView { count } = view.clone();
        Self { count }
    }
}

impl ddd::Model for WinnerCarrierBadgeModel {
    type View = WinnerCarrierBadgeView;
}
