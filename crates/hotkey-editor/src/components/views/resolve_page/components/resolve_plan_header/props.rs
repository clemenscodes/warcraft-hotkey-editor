use super::components::resolve_apply_button::ResolveApplyButtonProps;
use super::components::resolve_plan_summary::ResolvePlanSummaryProps;
use dioxus::prelude::*;

/// The plan header: the move/unresolved summary and the Apply button.
#[derive(Props, Clone, PartialEq)]
pub struct ResolvePlanHeaderProps {
    #[props(into)]
    pub moves_text: String,
    pub unresolved_count: usize,
    pub running: bool,
    pub on_apply: EventHandler<MouseEvent>,
}

impl From<&ResolvePlanHeaderProps> for ResolvePlanSummaryProps {
    fn from(props: &ResolvePlanHeaderProps) -> Self {
        Self {
            moves_text: props.moves_text.clone(),
            unresolved_count: props.unresolved_count,
        }
    }
}

impl From<&ResolvePlanHeaderProps> for ResolveApplyButtonProps {
    fn from(props: &ResolvePlanHeaderProps) -> Self {
        Self {
            running: props.running,
            onclick: props.on_apply,
        }
    }
}
