use dioxus::prelude::*;

/// The carrier-count badge when its ability wins the cell: gold.
#[derive(Props, Clone, PartialEq)]
pub struct WinnerCarrierBadgeProps {
    pub count: usize,
}
