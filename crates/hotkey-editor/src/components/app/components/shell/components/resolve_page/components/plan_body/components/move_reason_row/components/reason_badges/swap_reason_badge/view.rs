/// The published `View` contract mirroring [`SwapReasonBadgeProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SwapReasonBadgeView {
    pub label: String,
}

impl ddd::View for SwapReasonBadgeView {}
