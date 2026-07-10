use dioxus::prelude::*;

/// The "Swap" reason badge's label text, forwarded to the base `ReasonBadge` it composes.
#[derive(Props, Clone, PartialEq)]
pub struct SwapReasonBadgeProps {
    #[props(into)]
    pub label: String,
}
