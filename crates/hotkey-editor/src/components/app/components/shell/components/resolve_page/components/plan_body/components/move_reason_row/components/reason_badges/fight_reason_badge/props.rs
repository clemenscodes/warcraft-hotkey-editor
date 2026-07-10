use dioxus::prelude::*;

/// The "Fight" reason badge's label text, forwarded to the base `ReasonBadge` it composes.
#[derive(Props, Clone, PartialEq)]
pub struct FightReasonBadgeProps {
    #[props(into)]
    pub label: String,
}
