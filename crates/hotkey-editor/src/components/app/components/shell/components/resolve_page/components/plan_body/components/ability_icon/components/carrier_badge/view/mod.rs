/// The published `View` contract mirroring [`CarrierBadgeModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CarrierBadgeView {
    pub count: usize,
    pub is_winner: bool,
}

impl ddd::View for CarrierBadgeView {}
