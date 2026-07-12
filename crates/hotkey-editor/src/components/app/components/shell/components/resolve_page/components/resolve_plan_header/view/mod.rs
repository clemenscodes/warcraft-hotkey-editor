use super::ResolvePlanHeader;
use super::model::ResolvePlanHeaderModel;
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The published `View` for the resolve plan header region: the cascade summary text, the
/// Apply control's state and handler, and the move-category breadcrumb tabs. It is also the
/// resolve page frame's header region: it `impl Render` and renders the `ResolvePlanHeader`
/// once, so the page places the published `View` directly, with no ad-hoc region type.
#[derive(Clone, PartialEq, Default)]
pub struct ResolvePlanHeaderView {
    pub moves_text: String,
    pub unresolved_count: usize,
    pub running: bool,
    pub on_apply: EventHandler<MouseEvent>,
    pub breadcrumbs: Vec<BreadcrumbView>,
}

impl ddd::View for ResolvePlanHeaderView {}

impl Render for ResolvePlanHeaderView {
    type Model = ResolvePlanHeaderModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let moves_text = self.moves_text.clone();
        let unresolved_count = self.unresolved_count;
        let running = self.running;
        let on_apply = self.on_apply;
        let breadcrumbs = self.breadcrumbs.clone();
        rsx! {
            ResolvePlanHeader {
                moves_text,
                unresolved_count,
                running,
                on_apply,
                breadcrumbs,
            }
        }
    }
}
