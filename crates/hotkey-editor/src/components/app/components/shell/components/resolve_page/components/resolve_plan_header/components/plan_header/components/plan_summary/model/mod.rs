use super::view::PlanSummaryView;
use dioxus::prelude::*;

/// The plan title + counts block. `moves_text` is pre-built ("5 moves").
#[derive(Props, Clone, PartialEq)]
pub struct PlanSummaryModel {
    #[props(into)]
    pub moves_text: String,
    pub unresolved_count: usize,
}

impl From<&PlanSummaryView> for PlanSummaryModel {
    fn from(view: &PlanSummaryView) -> Self {
        let PlanSummaryView {
            moves_text,
            unresolved_count,
        } = view.clone();
        Self {
            moves_text,
            unresolved_count,
        }
    }
}

impl ddd::Model for PlanSummaryModel {
    type View = PlanSummaryView;
}
