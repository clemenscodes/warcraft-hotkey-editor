use super::components::reason_badge::ReasonBadgeProps;
use crate::components::views::resolve_page::logic::ReasonKind;
use dioxus::prelude::*;

/// The reason-badge row atop a move card.
#[derive(Props, Clone, PartialEq)]
pub struct MoveReasonRowProps {
    pub kind: ReasonKind,
    #[props(into)]
    pub label: String,
}

impl From<&MoveReasonRowProps> for ReasonBadgeProps {
    fn from(props: &MoveReasonRowProps) -> Self {
        Self {
            kind: props.kind,
            label: props.label.clone(),
        }
    }
}
