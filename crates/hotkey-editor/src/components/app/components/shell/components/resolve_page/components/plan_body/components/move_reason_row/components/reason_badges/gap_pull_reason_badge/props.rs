use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::components::reason_badges::shared::reason_badge::ReasonBadgeProps;
use dioxus::prelude::*;

/// The "GapPull" reason badge's label text, forwarded to the base `ReasonBadge` it composes.
#[derive(Props, Clone, PartialEq)]
pub struct GapPullReasonBadgeProps {
    #[props(into)]
    pub label: String,
}

impl From<&GapPullReasonBadgeProps> for ReasonBadgeProps {
    fn from(props: &GapPullReasonBadgeProps) -> Self {
        let label = props.label.clone();
        Self { label }
    }
}
