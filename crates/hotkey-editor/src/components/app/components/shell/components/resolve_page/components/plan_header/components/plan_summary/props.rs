use dioxus::prelude::*;

/// The plan title + counts block. `moves_text` is pre-built ("5 moves").
#[derive(Props, Clone, PartialEq)]
pub struct PlanSummaryProps {
    #[props(into)]
    pub moves_text: String,
    pub unresolved_count: usize,
}
