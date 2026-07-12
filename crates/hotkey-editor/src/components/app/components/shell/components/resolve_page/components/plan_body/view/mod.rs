use super::PlanBody;
use super::model::PlanBodyModel;
use crate::components::app::components::shell::components::resolve_page::presentation::{
    MoveSection, UnresolvedView,
};
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`PlanBodyModel`], threaded to this component as
/// data. It is also the resolve page frame's body region: it `impl Render` and renders the
/// `PlanBody` once, so the page places the published `View` directly, with no ad-hoc region
/// type.
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
            PlanBody { section, unresolved }
        }
    }
}
