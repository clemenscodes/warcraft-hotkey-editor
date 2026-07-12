use crate::components::app::components::shell::components::resolve_page::presentation::ReasonKind;

/// The published `View` contract mirroring [`MoveReasonBadgeModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MoveReasonBadgeView {
    pub kind: ReasonKind,
    pub label: String,
}

impl ddd::View for MoveReasonBadgeView {}
