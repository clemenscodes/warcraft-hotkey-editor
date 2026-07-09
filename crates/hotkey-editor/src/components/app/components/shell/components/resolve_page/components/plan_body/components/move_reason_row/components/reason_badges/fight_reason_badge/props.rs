use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::components::reason_badges::shared::reason_badge::ReasonBadgeProps;
use dioxus::prelude::*;

/// The "Fight" reason badge's label text, forwarded to the base `ReasonBadge` it composes.
#[derive(Props, Clone, PartialEq)]
pub struct FightReasonBadgeProps {
    #[props(into)]
    pub label: String,
}

impl From<&FightReasonBadgeProps> for ReasonBadgeProps {
    fn from(props: &FightReasonBadgeProps) -> Self {
        let label = props.label.clone();
        Self { label }
    }
}
