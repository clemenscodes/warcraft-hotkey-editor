/// The published `View` contract mirroring [`FightReasonBadgeProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FightReasonBadgeView {
    pub label: String,
}

impl ddd::View for FightReasonBadgeView {}
