use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::components::reason_badges::shared::reason_badge::ReasonBadgeProps;
use dioxus::prelude::*;

/// The "Stuck" reason badge's label text, forwarded to the base `ReasonBadge` it composes.
#[derive(Props, Clone, PartialEq)]
pub struct StuckReasonBadgeProps {
    #[props(into)]
    pub label: String,
}

impl From<&StuckReasonBadgeProps> for ReasonBadgeProps {
    fn from(props: &StuckReasonBadgeProps) -> Self {
        let label = props.label.clone();
        Self { label }
    }
}
