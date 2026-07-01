use super::components::resolve_plan_counts::ResolvePlanCountsProps;
use dioxus::prelude::*;

/// The plan title + counts block. `moves_text` is pre-built ("5 moves").
#[derive(Props, Clone, PartialEq)]
pub struct ResolvePlanSummaryProps {
    #[props(into)]
    pub moves_text: String,
    pub unresolved_count: usize,
}

impl From<&ResolvePlanSummaryProps> for ResolvePlanCountsProps {
    fn from(props: &ResolvePlanSummaryProps) -> Self {
        Self {
            moves_text: props.moves_text.clone(),
            unresolved_count: props.unresolved_count,
        }
    }
}
