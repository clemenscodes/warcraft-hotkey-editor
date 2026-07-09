use dioxus::prelude::*;

/// The carrier-count badge when its ability does not win the cell.
#[derive(Props, Clone, PartialEq)]
pub struct RegularCarrierBadgeProps {
    pub count: usize,
}
