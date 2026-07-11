use crate::components::app::components::shell::components::resolve_page::presentation::ReasonKind;

/// The published `View` contract mirroring [`MoveReasonRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MoveReasonRowView {
    pub kind: ReasonKind,
    pub label: String,
}

impl ddd::View for MoveReasonRowView {}
