use super::PlanBody;
use super::model::PlanBodyModel;
use crate::components::app::components::shell::components::resolve_page::presentation::{
    MoveSection, UnresolvedView,
};
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct PlanBodyView {
    pub section: Option<MoveSection>,
    pub unresolved: Vec<UnresolvedView>,
}

impl ddd::View for PlanBodyView {}

impl Render for PlanBodyView {
    type Model = PlanBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let section = self.section.clone();
        let unresolved = self.unresolved.clone();
        rsx! {
            PlanBody {
                section,
                unresolved,
            }
        }
    }
}
