use super::view::PlanHeaderView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PlanHeaderModel {
    #[props(into)]
    pub moves_text: String,
    pub unresolved_count: usize,
    pub running: bool,
    pub on_apply: EventHandler<MouseEvent>,
}

impl From<&PlanHeaderView> for PlanHeaderModel {
    fn from(view: &PlanHeaderView) -> Self {
        let PlanHeaderView {
            moves_text,
            unresolved_count,
            running,
            on_apply,
        } = view.clone();
        Self {
            moves_text,
            unresolved_count,
            running,
            on_apply,
        }
    }
}

impl ddd::Model for PlanHeaderModel {
    type View = PlanHeaderView;
}
