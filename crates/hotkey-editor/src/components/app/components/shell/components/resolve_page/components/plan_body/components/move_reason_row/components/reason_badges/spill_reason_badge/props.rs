use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::components::reason_badges::shared::reason_badge::ReasonBadgeProps;
use dioxus::prelude::*;

/// The "Spill" reason badge's label text, forwarded to the base `ReasonBadge` it composes.
#[derive(Props, Clone, PartialEq)]
pub struct SpillReasonBadgeProps {
    #[props(into)]
    pub label: String,
}

impl From<&SpillReasonBadgeProps> for ReasonBadgeProps {
    fn from(props: &SpillReasonBadgeProps) -> Self {
        let label = props.label.clone();
        Self { label }
    }
}
