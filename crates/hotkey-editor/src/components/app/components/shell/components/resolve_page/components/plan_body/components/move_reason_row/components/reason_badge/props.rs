use crate::components::app::components::shell::components::resolve_page::logic::ReasonKind;
use dioxus::prelude::*;

/// A move's reason badge: its label and the kind that colours it.
#[derive(Props, Clone, PartialEq)]
pub struct ReasonBadgeProps {
    pub kind: ReasonKind,
    #[props(into)]
    pub label: String,
}
