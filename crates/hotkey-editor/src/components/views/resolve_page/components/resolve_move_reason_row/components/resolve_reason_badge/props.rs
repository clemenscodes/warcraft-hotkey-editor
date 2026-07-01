use crate::components::views::resolve_page::logic::ResolveReasonKind;
use dioxus::prelude::*;

/// A move's reason badge: its label and the kind that colours it.
#[derive(Props, Clone, PartialEq)]
pub struct ResolveReasonBadgeProps {
    pub kind: ResolveReasonKind,
    #[props(into)]
    pub label: String,
}
