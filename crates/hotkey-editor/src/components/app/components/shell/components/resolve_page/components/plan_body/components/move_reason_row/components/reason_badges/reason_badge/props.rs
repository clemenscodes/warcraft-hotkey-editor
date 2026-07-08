use super::state::ReasonBadgeColor;
use dioxus::prelude::*;

/// The base reason badge: its label text and the colour it wears.
#[derive(Props, Clone, PartialEq)]
pub struct ReasonBadgeProps {
    pub color: ReasonBadgeColor,
    #[props(into)]
    pub label: String,
}
