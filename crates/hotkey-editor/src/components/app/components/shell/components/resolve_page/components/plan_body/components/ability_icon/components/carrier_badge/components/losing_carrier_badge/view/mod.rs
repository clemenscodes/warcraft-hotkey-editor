/// The published `View` contract mirroring [`LosingCarrierBadgeModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct LosingCarrierBadgeView {
    pub count: usize,
}

impl ddd::View for LosingCarrierBadgeView {}
