use super::view::CarrierBadgeView;
use dioxus::prelude::*;

/// The carrier-count badge on an ability icon; gold when it wins the cell.
#[derive(Props, Clone, PartialEq)]
pub struct CarrierBadgeProps {
    pub count: usize,
    pub is_winner: bool,
}

impl From<&CarrierBadgeView> for CarrierBadgeProps {
    fn from(view: &CarrierBadgeView) -> Self {
        let CarrierBadgeView { count, is_winner } = view.clone();
        Self { count, is_winner }
    }
}

impl ddd::Props for CarrierBadgeProps {
    type View = CarrierBadgeView;
}
