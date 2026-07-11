use super::view::MoveReasonRowView;
use crate::components::app::components::shell::components::resolve_page::presentation::ReasonKind;
use dioxus::prelude::*;

/// The reason-badge row atop a move card.
#[derive(Props, Clone, PartialEq)]
pub struct MoveReasonRowModel {
    pub kind: ReasonKind,
    #[props(into)]
    pub label: String,
}

impl From<&MoveReasonRowView> for MoveReasonRowModel {
    fn from(view: &MoveReasonRowView) -> Self {
        let MoveReasonRowView { kind, label } = view.clone();
        Self { kind, label }
    }
}

impl ddd::Model for MoveReasonRowModel {
    type View = MoveReasonRowView;
}
