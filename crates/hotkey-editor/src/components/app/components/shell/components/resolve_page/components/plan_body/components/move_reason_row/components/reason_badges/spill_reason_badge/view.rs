/// The published `View` contract mirroring [`SpillReasonBadgeProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SpillReasonBadgeView {
    pub label: String,
}

impl ddd::View for SpillReasonBadgeView {}
