use super::view::PlanUnresolvedView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct PlanUnresolvedModel {
    pub count: usize,
}

impl From<&PlanUnresolvedView> for PlanUnresolvedModel {
    fn from(view: &PlanUnresolvedView) -> Self {
        let PlanUnresolvedView { count } = view.clone();
        Self { count }
    }
}

impl ddd::Model for PlanUnresolvedModel {
    type View = PlanUnresolvedView;
}
