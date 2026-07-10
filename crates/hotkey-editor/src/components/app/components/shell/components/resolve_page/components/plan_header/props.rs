use super::view::PlanHeaderView;
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

impl From<&PlanHeaderView> for PlanHeaderProps {
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

impl ddd::Props for PlanHeaderProps {
    type View = PlanHeaderView;
}
