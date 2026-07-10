use super::view::PlanUnresolvedView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct PlanUnresolvedProps {
    pub count: usize,
}

impl From<&PlanUnresolvedView> for PlanUnresolvedProps {
    fn from(view: &PlanUnresolvedView) -> Self {
        let PlanUnresolvedView { count } = view.clone();
        Self { count }
    }
}

impl ddd::Props for PlanUnresolvedProps {
    type View = PlanUnresolvedView;
}
