/// The published `View` contract mirroring [`RegularCarrierBadgeProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct RegularCarrierBadgeView {
    pub count: usize,
}

impl ddd::View for RegularCarrierBadgeView {}
