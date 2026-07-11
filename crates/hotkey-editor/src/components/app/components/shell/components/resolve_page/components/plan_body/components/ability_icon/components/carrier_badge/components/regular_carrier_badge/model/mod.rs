use super::view::RegularCarrierBadgeView;
use dioxus::prelude::*;

/// The carrier-count badge when its ability does not win the cell.
#[derive(Props, Clone, PartialEq)]
pub struct RegularCarrierBadgeModel {
    pub count: usize,
}

impl From<&RegularCarrierBadgeView> for RegularCarrierBadgeModel {
    fn from(view: &RegularCarrierBadgeView) -> Self {
        let RegularCarrierBadgeView { count } = view.clone();
        Self { count }
    }
}

impl ddd::Model for RegularCarrierBadgeModel {
    type View = RegularCarrierBadgeView;
}
