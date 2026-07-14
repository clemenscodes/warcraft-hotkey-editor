use super::view::ResolvePlanHeaderView;
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ResolvePlanHeaderModel {
    #[props(into)]
    pub moves_text: String,
    pub unresolved_count: usize,
    pub running: bool,
    pub on_apply: EventHandler<MouseEvent>,
    pub breadcrumbs: Vec<BreadcrumbView>,
}

impl From<&ResolvePlanHeaderView> for ResolvePlanHeaderModel {
    fn from(view: &ResolvePlanHeaderView) -> Self {
        let ResolvePlanHeaderView {
            moves_text,
            unresolved_count,
            running,
            on_apply,
            breadcrumbs,
        } = view.clone();
        Self {
            moves_text,
            unresolved_count,
            running,
            on_apply,
            breadcrumbs,
        }
    }
}

impl ddd::Model for ResolvePlanHeaderModel {
    type View = ResolvePlanHeaderView;
}
