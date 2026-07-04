use super::components::apply_button::ApplyButtonProps;
use super::components::plan_summary::PlanSummaryProps;
use dioxus::prelude::*;

/// The plan header: the move/unresolved summary and the Apply button.
#[derive(Props, Clone, PartialEq)]
pub struct PlanHeaderProps {
    #[props(into)]
    pub moves_text: String,
    pub unresolved_count: usize,
    pub running: bool,
    pub on_apply: EventHandler<MouseEvent>,
}

impl From<&PlanHeaderProps> for PlanSummaryProps {
    fn from(props: &PlanHeaderProps) -> Self {
        Self {
            moves_text: props.moves_text.clone(),
            unresolved_count: props.unresolved_count,
        }
    }
}

impl From<&PlanHeaderProps> for ApplyButtonProps {
    fn from(props: &PlanHeaderProps) -> Self {
        Self {
            running: props.running,
            onclick: props.on_apply,
        }
    }
}
