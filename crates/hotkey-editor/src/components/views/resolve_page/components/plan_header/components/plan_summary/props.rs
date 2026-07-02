use super::components::plan_counts::PlanCountsProps;
use dioxus::prelude::*;

/// The plan title + counts block. `moves_text` is pre-built ("5 moves").
#[derive(Props, Clone, PartialEq)]
pub struct PlanSummaryProps {
    #[props(into)]
    pub moves_text: String,
    pub unresolved_count: usize,
}

impl From<&PlanSummaryProps> for PlanCountsProps {
    fn from(props: &PlanSummaryProps) -> Self {
        Self {
            moves_text: props.moves_text.clone(),
            unresolved_count: props.unresolved_count,
        }
    }
}
