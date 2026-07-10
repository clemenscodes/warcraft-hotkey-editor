use super::view::RegularCarrierBadgeView;
use dioxus::prelude::*;

/// The carrier-count badge when its ability does not win the cell.
#[derive(Props, Clone, PartialEq)]
pub struct RegularCarrierBadgeProps {
    pub count: usize,
}

impl From<&RegularCarrierBadgeView> for RegularCarrierBadgeProps {
    fn from(view: &RegularCarrierBadgeView) -> Self {
        let RegularCarrierBadgeView { count } = view.clone();
        Self { count }
    }
}

impl ddd::Props for RegularCarrierBadgeProps {
    type View = RegularCarrierBadgeView;
}
