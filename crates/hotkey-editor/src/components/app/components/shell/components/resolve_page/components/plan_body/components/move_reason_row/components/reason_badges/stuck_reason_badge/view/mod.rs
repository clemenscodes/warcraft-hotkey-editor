/// The published `View` contract mirroring [`StuckReasonBadgeModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct StuckReasonBadgeView {
    pub label: String,
}

impl ddd::View for StuckReasonBadgeView {}
