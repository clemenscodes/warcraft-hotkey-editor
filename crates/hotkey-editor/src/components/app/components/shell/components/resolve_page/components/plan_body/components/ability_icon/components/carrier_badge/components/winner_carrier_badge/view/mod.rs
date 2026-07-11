/// The published `View` contract mirroring [`WinnerCarrierBadgeModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct WinnerCarrierBadgeView {
    pub count: usize,
}

impl ddd::View for WinnerCarrierBadgeView {}
