use crate::components::app::components::shell::components::resolve_page::logic::ReasonKind;

/// The published `View` contract mirroring [`MoveReasonRowProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MoveReasonRowView {
    pub kind: ReasonKind,
    pub label: String,
}

impl ddd::View for MoveReasonRowView {}
