use super::view::ResolvePlanHeaderView;
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;
use dioxus::prelude::*;

/// The resolve plan header region: the cascade summary and Apply control, plus the
/// move-category breadcrumb tabs. The bar's assistive-tech name is fixed ("Move categories"),
/// so it is the region's own identity, not a field.
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
