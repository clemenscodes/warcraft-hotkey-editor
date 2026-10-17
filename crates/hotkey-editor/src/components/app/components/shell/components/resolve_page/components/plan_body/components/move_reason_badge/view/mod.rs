use crate::components::app::components::shell::components::resolve_page::presentation::ReasonKind;

#[derive(Clone, PartialEq)]
pub struct MoveReasonBadgeView {
    pub kind: ReasonKind,
    pub label: String,
}

impl ddd::Layered for MoveReasonBadgeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MoveReasonBadgeView {}
