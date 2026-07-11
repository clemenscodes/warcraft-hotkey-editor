use super::view::PlanBodyView;
use crate::components::app::components::shell::components::resolve_page::presentation::{
    MoveSection, UnresolvedView,
};
use dioxus::prelude::*;

/// The scrollable plan body: the active move section (absent when the plan has only
/// unresolved abilities) and every unresolved ability.
#[derive(Props, Clone, PartialEq)]
pub struct PlanBodyModel {
    pub section: Option<MoveSection>,
    pub unresolved: Vec<UnresolvedView>,
}

impl From<&PlanBodyView> for PlanBodyModel {
    fn from(view: &PlanBodyView) -> Self {
        let PlanBodyView {
            section,
            unresolved,
        } = view.clone();
        Self {
            section,
            unresolved,
        }
    }
}

impl ddd::Model for PlanBodyModel {
    type View = PlanBodyView;
}
