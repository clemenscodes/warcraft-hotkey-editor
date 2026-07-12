use super::view::MoveReasonBadgeView;
use crate::components::app::components::shell::components::resolve_page::presentation::ReasonKind;
use dioxus::prelude::*;

/// The reason-badge row atop a move card.
#[derive(Props, Clone, PartialEq)]
pub struct MoveReasonBadgeModel {
    pub kind: ReasonKind,
    #[props(into)]
    pub label: String,
}

impl From<&MoveReasonBadgeView> for MoveReasonBadgeModel {
    fn from(view: &MoveReasonBadgeView) -> Self {
        let MoveReasonBadgeView { kind, label } = view.clone();
        Self { kind, label }
    }
}

impl ddd::Model for MoveReasonBadgeModel {
    type View = MoveReasonBadgeView;
}
