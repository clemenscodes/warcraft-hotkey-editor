use dioxus::prelude::*;

/// The "Stuck" reason badge's label text, forwarded to the base `ReasonBadge` it composes.
#[derive(Props, Clone, PartialEq)]
pub struct StuckReasonBadgeProps {
    #[props(into)]
    pub label: String,
}
