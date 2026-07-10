use dioxus::prelude::*;

/// The "Spill" reason badge's label text, forwarded to the base `ReasonBadge` it composes.
#[derive(Props, Clone, PartialEq)]
pub struct SpillReasonBadgeProps {
    #[props(into)]
    pub label: String,
}
