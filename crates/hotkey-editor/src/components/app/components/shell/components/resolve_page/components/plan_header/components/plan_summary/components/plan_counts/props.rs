use super::view::PlanCountsView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct PlanCountsProps {
    #[props(into)]
    pub moves_text: String,
    pub unresolved_count: usize,
}

impl From<&PlanCountsView> for PlanCountsProps {
    fn from(view: &PlanCountsView) -> Self {
        let PlanCountsView {
            moves_text,
            unresolved_count,
        } = view.clone();
        Self {
            moves_text,
            unresolved_count,
        }
    }
}

impl ddd::Props for PlanCountsProps {
    type View = PlanCountsView;
}
