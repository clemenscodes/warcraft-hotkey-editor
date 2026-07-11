use super::view::WinnerCarrierBadgeView;
use dioxus::prelude::*;

/// The carrier-count badge when its ability wins the cell: gold.
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
