use super::view::MoveReasonRowView;
use crate::components::app::components::shell::components::resolve_page::logic::ReasonKind;
use dioxus::prelude::*;

/// The reason-badge row atop a move card.
#[derive(Props, Clone, PartialEq)]
pub struct MoveReasonRowProps {
    pub kind: ReasonKind,
    #[props(into)]
    pub label: String,
}

impl From<&MoveReasonRowView> for MoveReasonRowProps {
    fn from(view: &MoveReasonRowView) -> Self {
        let MoveReasonRowView { kind, label } = view.clone();
        Self { kind, label }
    }
}

impl ddd::Props for MoveReasonRowProps {
    type View = MoveReasonRowView;
}
