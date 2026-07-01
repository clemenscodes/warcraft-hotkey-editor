use super::components::resolve_reason_badge::ResolveReasonBadgeProps;
use crate::components::views::resolve_page::logic::ResolveReasonKind;
use dioxus::prelude::*;

/// The reason-badge row atop a move card.
#[derive(Props, Clone, PartialEq)]
pub struct ResolveMoveReasonRowProps {
    pub kind: ResolveReasonKind,
    #[props(into)]
    pub label: String,
}

impl From<&ResolveMoveReasonRowProps> for ResolveReasonBadgeProps {
    fn from(props: &ResolveMoveReasonRowProps) -> Self {
        Self {
            kind: props.kind,
            label: props.label.clone(),
        }
    }
}
