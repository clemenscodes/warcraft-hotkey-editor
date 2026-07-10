/// The published `View` contract mirroring [`GapPullReasonBadgeProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct GapPullReasonBadgeView {
    pub label: String,
}

impl ddd::View for GapPullReasonBadgeView {}
