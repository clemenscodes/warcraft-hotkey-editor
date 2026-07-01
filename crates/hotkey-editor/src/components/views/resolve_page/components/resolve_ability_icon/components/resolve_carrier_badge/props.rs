use dioxus::prelude::*;

/// The carrier-count badge on an ability icon; gold when it wins the cell.
#[derive(Props, Clone, PartialEq)]
pub struct ResolveCarrierBadgeProps {
    pub count: usize,
    pub is_winner: bool,
}
