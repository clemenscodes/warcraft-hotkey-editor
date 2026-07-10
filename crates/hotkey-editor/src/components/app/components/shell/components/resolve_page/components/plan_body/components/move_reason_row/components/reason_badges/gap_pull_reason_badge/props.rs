use dioxus::prelude::*;

/// The "GapPull" reason badge's label text, forwarded to the base `ReasonBadge` it composes.
#[derive(Props, Clone, PartialEq)]
pub struct GapPullReasonBadgeProps {
    #[props(into)]
    pub label: String,
}
