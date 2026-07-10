/// The published `View` contract mirroring [`ReasonBadgeProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ReasonBadgeView {
    pub label: String,
}

impl ddd::View for ReasonBadgeView {}
