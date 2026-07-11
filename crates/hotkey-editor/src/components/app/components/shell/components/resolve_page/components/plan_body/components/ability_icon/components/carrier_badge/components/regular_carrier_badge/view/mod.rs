/// The published `View` contract mirroring [`RegularCarrierBadgeModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct RegularCarrierBadgeView {
    pub count: usize,
}

impl ddd::View for RegularCarrierBadgeView {}
